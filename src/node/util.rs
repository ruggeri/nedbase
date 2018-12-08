pub(super) fn search_sorted_strings_for_str(
  strings: &[String],
  target_value: &str,
) -> Result<usize, usize> {
  let search_fn = |value: &String| value.as_str().cmp(target_value);
  strings.binary_search_by(search_fn)
}
