#[hotpath::measure_all]
impl RichTextEditor {
  pub fn set_search_highlights(&mut self, highlights: Vec<Range<DocumentOffset>>, active: Option<usize>, cx: &mut Context<Self>) {
    self.search_highlights = highlights;
    self.active_search_highlight = active.filter(|ix| *ix < self.search_highlights.len());
    cx.notify();
  }

  pub fn clear_search_highlights(&mut self, cx: &mut Context<Self>) {
    self.search_highlights.clear();
    self.active_search_highlight = None;
    cx.notify();
  }

  pub fn set_active_search_highlight(&mut self, active: Option<usize>, cx: &mut Context<Self>) {
    self.active_search_highlight = active.filter(|ix| *ix < self.search_highlights.len());
    if let Some(ix) = self.active_search_highlight {
      let range = self.search_highlights[ix].clone();
      self.pending_snap_to_paragraph = Some((range.start.paragraph, 3));
    }
    cx.notify();
  }

  pub fn replace_active_search_highlight(&mut self, replacement: &str, cx: &mut Context<Self>) -> bool {
    let Some(ix) = self.active_search_highlight else {
      return false;
    };
    let Some(range) = self.search_highlights.get(ix).cloned() else {
      return false;
    };

    self.selection = EditorSelection {
      anchor: range.start,
      head: range.end,
    };
    self.apply_document_edit(cx, |editor, cx| {
      editor.insert_text(replacement, cx);
    });
    self.clear_search_highlights(cx);
    true
  }

  pub fn replace_all_search_highlights(&mut self, replacement: &str, cx: &mut Context<Self>) -> usize {
    let mut ranges = std::mem::take(&mut self.search_highlights)
      .into_iter()
      .filter(|range| self.search_highlight_range_is_valid(range))
      .collect::<Vec<_>>();
    if ranges.is_empty() {
      self.active_search_highlight = None;
      cx.notify();
      return 0;
    }

    ranges.sort_by(|left, right| right.start.cmp(&left.start).then_with(|| right.end.cmp(&left.end)));
    let count = ranges.len();
    let paragraph_count = self.document.paragraphs.len();
    self.apply_document_edit_with_capture_range(cx, Some(0..paragraph_count), |editor, cx| {
      let mut final_caret = None;
      for range in ranges {
        if range.start.paragraph == range.end.paragraph {
          let paragraph_ix = range.start.paragraph;
          let styles = editor
            .document
            .paragraphs
            .get(paragraph_ix)
            .and_then(|paragraph| {
              let (run_ix, _) = run_containing(paragraph, range.start.byte);
              paragraph.runs.get(run_ix).map(|run| run.styles)
            })
            .unwrap_or_default();
          delete_range_in_paragraph(&mut editor.document, paragraph_ix, range.start.byte..range.end.byte);
          insert_text_at(&mut editor.document, paragraph_ix, range.start.byte, replacement, styles);
          let caret = DocumentOffset {
            paragraph: paragraph_ix,
            byte: range.start.byte + replacement.len(),
          };
          final_caret = Some(caret);
        } else {
          editor.selection = EditorSelection {
            anchor: range.start,
            head: range.end,
          };
          editor.insert_text(replacement, cx);
          final_caret = Some(editor.selection.head);
        }
      }
      if let Some(caret) = final_caret {
        editor.selection = EditorSelection { anchor: caret, head: caret };
      }
      editor.after_text_mutation(cx);
    });
    self.search_highlights.clear();
    self.active_search_highlight = None;
    cx.notify();
    count
  }

  fn search_highlight_range_is_valid(&self, range: &Range<DocumentOffset>) -> bool {
    if range.start > range.end || range.end.paragraph >= self.document.paragraphs.len() {
      return false;
    }
    let Some(start_paragraph) = self.document.paragraphs.get(range.start.paragraph) else {
      return false;
    };
    let Some(end_paragraph) = self.document.paragraphs.get(range.end.paragraph) else {
      return false;
    };
    range.start.byte <= paragraph_text_len(start_paragraph) && range.end.byte <= paragraph_text_len(end_paragraph)
  }
}
