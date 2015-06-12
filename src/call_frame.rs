//! A CallFrame contains information about a location where code is being
//! executed.
//!
//! A CallFrame is also used for storing values and variables for a certain
//! scope. This makes it easy to remove those values again when unwinding the
//! call stack: simply remove the CallFrame and everything else is also removed.

use compiled_code::RcCompiledCode;
use register::Register;
use variable_scope::VariableScope;

/// Structure for storing call frame data.
pub struct CallFrame {
    /// The name of the CallFrame, usually the same as the method name.
    pub name: String,

    /// The full path to the file being executed.
    pub file: String,

    /// The line number being executed.
    pub line: usize,

    /// An optional parent CallFrame.
    pub parent: Option<Box<CallFrame>>,

    /// Register for storing temporary values.
    pub register: Register,

    /// Storage for local variables.
    pub variables: VariableScope
}

impl CallFrame {
    /// Creates a basic CallFrame with only details such as the name, file and
    /// line number set.
    ///
    /// # Examples
    ///
    ///     let frame = CallFrame::new("(main)", "main.aeon", 1);
    ///
    pub fn new(name: String, file: String, line: usize) -> CallFrame {
        let frame = CallFrame {
            name: name,
            file: file,
            line: line,
            parent: None,
            register: Register::new(),
            variables: VariableScope::new()
        };

        frame
    }

    /// Creates a new CallFrame from a CompiledCode
    pub fn from_code(code: RcCompiledCode) -> CallFrame {
        CallFrame::new(code.name.clone(), code.file.clone(), code.line)
    }

    /// Boxes and sets the current frame's parent.
    pub fn set_parent(&mut self, parent: CallFrame) {
        self.parent = Some(Box::new(parent));
    }

    /// Calls the supplied closure for the current and any parent frames.
    ///
    /// The closure takes a single argument: a reference to the CallFrame
    /// currently being processed.
    ///
    /// # Examples
    ///
    ///     some_child_frame.each_frame(|frame| {
    ///         println!("Frame: {}", frame.name);
    ///     });
    ///
    pub fn each_frame<F>(&self, mut closure: F) where F : FnMut(&CallFrame) {
        let mut frame = self;

        closure(frame);

        while frame.parent.is_some() {
            frame = frame.parent.as_ref().unwrap();

            closure(frame);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use compiled_code::CompiledCode;

    #[test]
    fn test_new() {
        let frame = CallFrame
            ::new("foo".to_string(), "test.aeon".to_string(), 1);

        assert_eq!(frame.name, "foo".to_string());
        assert_eq!(frame.file, "test.aeon".to_string());
        assert_eq!(frame.line, 1);
    }

    #[test]
    fn test_from_code() {
        let code = CompiledCode
            ::with_rc("foo".to_string(), "test.aeon".to_string(), 1, vec![]);

        let frame = CallFrame::from_code(code);

        assert_eq!(frame.name, "foo".to_string());
        assert_eq!(frame.file, "test.aeon".to_string());
        assert_eq!(frame.line, 1);
    }

    #[test]
    fn test_set_parent() {
        let frame1 = CallFrame
            ::new("foo".to_string(), "test.aeon".to_string(), 1);

        let mut frame2 = CallFrame
            ::new("bar".to_string(), "baz.aeon".to_string(), 1);

        frame2.set_parent(frame1);

        assert!(frame2.parent.is_some());
    }

    #[test]
    fn test_each_frame() {
        let frame1 = CallFrame
            ::new("foo".to_string(), "test.aeon".to_string(), 1);

        let mut frame2 = CallFrame
            ::new("bar".to_string(), "baz.aeon".to_string(), 1);

        let mut names: Vec<String> = vec![];

        frame2.set_parent(frame1);

        frame2.each_frame(|frame| {
            names.push(frame.name.clone());
        });

        assert_eq!(names[0], "bar".to_string());
        assert_eq!(names[1], "foo".to_string());
    }
}
