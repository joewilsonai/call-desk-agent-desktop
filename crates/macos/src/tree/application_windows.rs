//! Modified by Joe Wilson for Call Desk: macOS window discovery and strict native-ID targeting.
use agent_desktop_core::AdapterError;
use std::time::Instant;

use super::AXElement;

pub(crate) fn read(app: &AXElement, deadline: Instant) -> Result<Vec<AXElement>, AdapterError> {
    collect(
        |attribute| super::surface_read::elements(app, attribute, deadline),
        |attribute| super::surface_read::element(app, attribute, deadline),
        |element| {
            Ok(
                super::surface_read::string(element, "AXRole", deadline)?.as_deref()
                    == Some("AXWindow"),
            )
        },
        super::same_element,
    )
}

fn collect<T>(
    mut read_array: impl FnMut(&str) -> Result<Vec<T>, AdapterError>,
    mut read_element: impl FnMut(&str) -> Result<Option<T>, AdapterError>,
    mut is_window: impl FnMut(&T) -> Result<bool, AdapterError>,
    same: impl Fn(&T, &T) -> bool,
) -> Result<Vec<T>, AdapterError> {
    let mut windows = Vec::new();
    let mut add = |candidate| -> Result<(), AdapterError> {
        if is_window(&candidate)? && !windows.iter().any(|window| same(window, &candidate)) {
            windows.push(candidate);
        }
        Ok(())
    };
    for candidate in read_array("AXWindows")? {
        add(candidate)?;
    }
    for attribute in ["AXMainWindow", "AXFocusedWindow"] {
        if let Some(candidate) = read_element(attribute)? {
            add(candidate)?;
        }
    }
    for candidate in read_array("AXChildren")? {
        add(candidate)?;
    }
    Ok(windows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_desktop_core::ErrorCode;

    #[test]
    fn supplements_desktop_only_window_list_with_main_and_focused_windows() {
        let windows = collect(
            |attribute| {
                Ok(if attribute == "AXWindows" {
                    vec![0]
                } else {
                    vec![0, 3]
                })
            },
            |attribute| Ok(Some(if attribute == "AXMainWindow" { 1 } else { 2 })),
            |candidate| Ok(*candidate != 0),
            |a, b| a == b,
        )
        .unwrap();
        assert_eq!(windows, vec![1, 2, 3]);
    }

    #[test]
    fn duplicate_window_sources_keep_one_candidate_in_order() {
        let windows = collect(
            |_| Ok(vec![1, 2]),
            |_| Ok(Some(1)),
            |_| Ok(true),
            |a, b| a == b,
        )
        .unwrap();
        assert_eq!(windows, vec![1, 2]);
    }

    #[test]
    fn secondary_source_permission_error_does_not_return_partial_windows() {
        let error = collect(
            |_| Ok(vec![1]),
            |_| Err(AdapterError::new(ErrorCode::PermDenied, "unavailable")),
            |_| Ok(true),
            |a, b| a == b,
        )
        .unwrap_err();
        assert_eq!(error.code, ErrorCode::PermDenied);
    }

    #[test]
    fn role_read_error_is_preserved() {
        let error = collect(
            |_| Ok(vec![1]),
            |_| Ok(None),
            |_| Err(AdapterError::timeout("window role")),
            |a, b| a == b,
        )
        .unwrap_err();
        assert_eq!(error.code, ErrorCode::Timeout);
    }
}
