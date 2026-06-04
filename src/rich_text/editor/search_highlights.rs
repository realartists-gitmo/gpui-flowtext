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
    let ranges = std::mem::take(&mut self.search_highlights);
    let count = ranges.len();
    if ranges.is_empty() {
      self.active_search_highlight = None;
      cx.notify();
      return 0;
    }

    let paragraph_count = self.document.paragraphs.len();
    self.apply_document_edit_with_capture_range(cx, Some(0..paragraph_count), |editor, cx| {
      for range in ranges.into_iter().rev() {
        editor.selection = EditorSelection {
          anchor: range.start,
          head: range.end,
        };
        editor.insert_text(replacement, cx);
      }
    });
    self.search_highlights.clear();
    self.active_search_highlight = None;
    cx.notify();
    count
  }
}
