
pub enum ValidationType {
  // No validation.
  None,

  // Validate success codes (only 2xx).
  SuccessCodes,

  // Validate success codes and redirection codes (only 2xx and 3xx).
  SuccessAndRedirectCodes,

  // Validate only the given status codes.
  CustomCodes(Vec<i16>),

}

impl ValidationType {
  pub fn status_code(&self) -> Vec<i16> {
    match self {
      ValidationType::SuccessCodes => (200..300).collect(),
      ValidationType::SuccessAndRedirectCodes => (200..400).collect(),
      ValidationType::CustomCodes(codes) => codes.to_vec(),
      ValidationType::None => Vec::new(),
    }
  }
}

impl PartialEq for ValidationType {
  fn eq(&self, other: &ValidationType) -> bool {
        match (self, other) {
          (ValidationType::None, ValidationType::None) | 
          (ValidationType::SuccessCodes, ValidationType::SuccessCodes) |
          (ValidationType::SuccessAndRedirectCodes, ValidationType::SuccessAndRedirectCodes) => true,
          (ValidationType::CustomCodes(codes1), ValidationType::CustomCodes(codes2)) => codes1 == codes2,
           _ => false
        }
  }
}

