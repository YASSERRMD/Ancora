/// License declaration for marketplace extensions.
///
/// Every extension must declare a valid SPDX license identifier. Some license
/// classes may be blocked by the registry or by enterprise install policies.

#[derive(Debug, Clone, PartialEq)]
pub struct LicenseDeclaration {
    /// SPDX expression, e.g. "Apache-2.0", "MIT OR Apache-2.0".
    pub spdx_expression: String,
    /// URL pointing to the full license text.
    pub license_url: Option<String>,
    /// Whether the source code is available under this license.
    pub is_open_source: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LicenseClass {
    /// Standard permissive open-source (MIT, Apache-2.0, BSD-*).
    Permissive,
    /// Copyleft open-source (GPL-*, LGPL-*, AGPL-*).
    Copyleft,
    /// Source-available but not OSI-approved.
    SourceAvailable,
    /// Proprietary / closed-source.
    Proprietary,
    /// Unknown or unrecognized SPDX expression.
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LicenseError {
    EmptySpdxExpression,
    LicenseClassBlocked(LicenseClass),
    DeclarationMissing,
}

impl std::fmt::Display for LicenseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LicenseError::EmptySpdxExpression => write!(f, "SPDX expression must not be empty"),
            LicenseError::LicenseClassBlocked(c) => {
                write!(f, "license class {:?} is blocked by policy", c)
            }
            LicenseError::DeclarationMissing => write!(f, "license declaration is missing"),
        }
    }
}

impl LicenseDeclaration {
    pub fn new(spdx_expression: impl Into<String>, is_open_source: bool) -> Result<Self, LicenseError> {
        let spdx_expression = spdx_expression.into();
        if spdx_expression.is_empty() {
            return Err(LicenseError::EmptySpdxExpression);
        }
        Ok(LicenseDeclaration {
            spdx_expression,
            license_url: None,
            is_open_source,
        })
    }

    /// Classify this license based on known SPDX identifiers.
    pub fn classify(&self) -> LicenseClass {
        let expr = self.spdx_expression.as_str();
        if is_permissive(expr) {
            LicenseClass::Permissive
        } else if is_copyleft(expr) {
            LicenseClass::Copyleft
        } else if expr == "PROPRIETARY" || expr == "Proprietary" {
            LicenseClass::Proprietary
        } else {
            LicenseClass::Unknown
        }
    }
}

fn is_permissive(expr: &str) -> bool {
    let known = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Unlicense", "0BSD"];
    known.iter().any(|&k| expr.contains(k))
}

fn is_copyleft(expr: &str) -> bool {
    let known = ["GPL-2.0", "GPL-3.0", "LGPL-2.0", "LGPL-2.1", "LGPL-3.0", "AGPL-3.0"];
    known.iter().any(|&k| expr.contains(k))
}

/// Validate that a license declaration is present and allowed by policy.
pub fn require_license(
    declaration: Option<&LicenseDeclaration>,
    block_proprietary: bool,
) -> Result<(), LicenseError> {
    let decl = declaration.ok_or(LicenseError::DeclarationMissing)?;
    if decl.spdx_expression.is_empty() {
        return Err(LicenseError::EmptySpdxExpression);
    }
    if block_proprietary && decl.classify() == LicenseClass::Proprietary {
        return Err(LicenseError::LicenseClassBlocked(LicenseClass::Proprietary));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apache_is_permissive() {
        let d = LicenseDeclaration::new("Apache-2.0", true).unwrap();
        assert_eq!(d.classify(), LicenseClass::Permissive);
    }

    #[test]
    fn missing_license_blocked() {
        assert_eq!(require_license(None, false), Err(LicenseError::DeclarationMissing));
    }

    #[test]
    fn proprietary_blocked_by_policy() {
        let d = LicenseDeclaration::new("PROPRIETARY", false).unwrap();
        assert_eq!(
            require_license(Some(&d), true),
            Err(LicenseError::LicenseClassBlocked(LicenseClass::Proprietary))
        );
    }
}
