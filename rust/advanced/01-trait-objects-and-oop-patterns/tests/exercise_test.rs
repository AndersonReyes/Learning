use advanced_01_trait_objects_and_oop_patterns::{
    count_by_type, eval_all, run_tasks_in_priority_order, run_turnstile, Add, Circle, Div,
    EmailNotifier, Event, Expr, LoggingDecorator, Mul, Neg, Notifier, Num, Rectangle, Shape,
    SmsDecorator, Square, Sub, Task,
};

// --- eval_all / Expr ------------------------------------------------------------------

#[test]
fn eval_num_is_itself() {
    let exprs: Vec<Box<dyn Expr>> = vec![Box::new(Num(5.0))];
    assert_eq!(eval_all(exprs), vec![Ok(5.0)]);
}

#[test]
fn eval_add_sums_operands() {
    let exprs: Vec<Box<dyn Expr>> = vec![Box::new(Add(Box::new(Num(2.0)), Box::new(Num(3.0))))];
    assert_eq!(eval_all(exprs), vec![Ok(5.0)]);
}

#[test]
fn eval_sub_subtracts_operands() {
    let exprs: Vec<Box<dyn Expr>> = vec![Box::new(Sub(Box::new(Num(5.0)), Box::new(Num(3.0))))];
    assert_eq!(eval_all(exprs), vec![Ok(2.0)]);
}

#[test]
fn eval_nested_add_and_mul() {
    // (2 + 3) * 4 = 20
    let exprs: Vec<Box<dyn Expr>> = vec![Box::new(Mul(
        Box::new(Add(Box::new(Num(2.0)), Box::new(Num(3.0)))),
        Box::new(Num(4.0)),
    ))];
    assert_eq!(eval_all(exprs), vec![Ok(20.0)]);
}

#[test]
fn eval_div_normal() {
    let exprs: Vec<Box<dyn Expr>> = vec![Box::new(Div(Box::new(Num(10.0)), Box::new(Num(2.0))))];
    assert_eq!(eval_all(exprs), vec![Ok(5.0)]);
}

#[test]
fn eval_div_by_zero_is_err() {
    let exprs: Vec<Box<dyn Expr>> = vec![Box::new(Div(Box::new(Num(10.0)), Box::new(Num(0.0))))];
    assert_eq!(eval_all(exprs), vec![Err("division by zero".to_string())]);
}

#[test]
fn eval_neg_negates() {
    let exprs: Vec<Box<dyn Expr>> = vec![Box::new(Neg(Box::new(Num(7.0))))];
    assert_eq!(eval_all(exprs), vec![Ok(-7.0)]);
}

#[test]
fn eval_division_by_zero_propagates_through_parent() {
    // (1 / 0) + 5 -- the error from the inner Div propagates out of Add.
    let exprs: Vec<Box<dyn Expr>> = vec![Box::new(Add(
        Box::new(Div(Box::new(Num(1.0)), Box::new(Num(0.0)))),
        Box::new(Num(5.0)),
    ))];
    assert_eq!(eval_all(exprs), vec![Err("division by zero".to_string())]);
}

#[test]
fn eval_all_preserves_order_and_independence() {
    let exprs: Vec<Box<dyn Expr>> = vec![
        Box::new(Num(1.0)),
        Box::new(Div(Box::new(Num(1.0)), Box::new(Num(0.0)))),
        Box::new(Num(2.0)),
    ];
    assert_eq!(
        eval_all(exprs),
        vec![Ok(1.0), Err("division by zero".to_string()), Ok(2.0)]
    );
}

#[test]
fn eval_all_empty_is_empty() {
    let exprs: Vec<Box<dyn Expr>> = vec![];
    assert_eq!(eval_all(exprs), Vec::<Result<f64, String>>::new());
}

// --- run_turnstile ---------------------------------------------------------------------

#[test]
fn turnstile_empty_events_is_empty() {
    assert_eq!(run_turnstile(&[]), Vec::<&str>::new());
}

#[test]
fn turnstile_push_while_locked_stays_locked() {
    assert_eq!(run_turnstile(&[Event::Push]), vec!["Locked"]);
}

#[test]
fn turnstile_coin_unlocks() {
    assert_eq!(run_turnstile(&[Event::Coin]), vec!["Unlocked"]);
}

#[test]
fn turnstile_coin_then_push_locks_again() {
    assert_eq!(
        run_turnstile(&[Event::Coin, Event::Push]),
        vec!["Unlocked", "Locked"]
    );
}

#[test]
fn turnstile_extra_coin_while_unlocked_stays_unlocked() {
    assert_eq!(
        run_turnstile(&[Event::Coin, Event::Coin, Event::Push, Event::Push]),
        vec!["Unlocked", "Unlocked", "Locked", "Locked"]
    );
}

#[test]
fn turnstile_repeated_push_while_locked_stays_locked() {
    assert_eq!(
        run_turnstile(&[Event::Push, Event::Push, Event::Coin]),
        vec!["Locked", "Locked", "Unlocked"]
    );
}

#[test]
fn turnstile_full_cycle() {
    assert_eq!(
        run_turnstile(&[
            Event::Coin,
            Event::Push,
            Event::Push,
            Event::Coin,
            Event::Coin,
            Event::Push,
        ]),
        vec!["Unlocked", "Locked", "Locked", "Unlocked", "Unlocked", "Locked"]
    );
}

// --- run_tasks_in_priority_order --------------------------------------------------------

struct Labeled {
    label: &'static str,
    priority: i32,
}

impl Task for Labeled {
    fn priority(&self) -> i32 {
        self.priority
    }

    fn run(&self) -> String {
        self.label.to_string()
    }
}

#[test]
fn tasks_empty_is_empty() {
    let tasks: Vec<Box<dyn Task>> = vec![];
    assert_eq!(run_tasks_in_priority_order(tasks), Vec::<String>::new());
}

#[test]
fn tasks_single_task() {
    let tasks: Vec<Box<dyn Task>> = vec![Box::new(Labeled {
        label: "only",
        priority: 0,
    })];
    assert_eq!(run_tasks_in_priority_order(tasks), vec!["only"]);
}

#[test]
fn tasks_sorted_by_priority_ascending() {
    let tasks: Vec<Box<dyn Task>> = vec![
        Box::new(Labeled {
            label: "low",
            priority: 5,
        }),
        Box::new(Labeled {
            label: "high",
            priority: 1,
        }),
        Box::new(Labeled {
            label: "medium",
            priority: 3,
        }),
    ];
    assert_eq!(
        run_tasks_in_priority_order(tasks),
        vec!["high", "medium", "low"]
    );
}

#[test]
fn tasks_equal_priority_preserves_original_order() {
    let tasks: Vec<Box<dyn Task>> = vec![
        Box::new(Labeled {
            label: "first",
            priority: 1,
        }),
        Box::new(Labeled {
            label: "second",
            priority: 1,
        }),
        Box::new(Labeled {
            label: "third",
            priority: 1,
        }),
    ];
    assert_eq!(
        run_tasks_in_priority_order(tasks),
        vec!["first", "second", "third"]
    );
}

#[test]
fn tasks_negative_priorities_run_first() {
    let tasks: Vec<Box<dyn Task>> = vec![
        Box::new(Labeled {
            label: "zero",
            priority: 0,
        }),
        Box::new(Labeled {
            label: "negative",
            priority: -10,
        }),
        Box::new(Labeled {
            label: "positive",
            priority: 10,
        }),
    ];
    assert_eq!(
        run_tasks_in_priority_order(tasks),
        vec!["negative", "zero", "positive"]
    );
}

#[test]
fn tasks_mixed_ties_preserve_relative_order_within_each_priority() {
    let tasks: Vec<Box<dyn Task>> = vec![
        Box::new(Labeled {
            label: "a-2",
            priority: 2,
        }),
        Box::new(Labeled {
            label: "a-1",
            priority: 1,
        }),
        Box::new(Labeled {
            label: "b-1",
            priority: 1,
        }),
        Box::new(Labeled {
            label: "b-2",
            priority: 2,
        }),
    ];
    assert_eq!(
        run_tasks_in_priority_order(tasks),
        vec!["a-1", "b-1", "a-2", "b-2"]
    );
}

// --- count_by_type -----------------------------------------------------------------------

#[test]
fn count_by_type_empty_slice_is_empty_map() {
    let shapes: Vec<Box<dyn Shape>> = vec![];
    assert_eq!(count_by_type(&shapes).len(), 0);
}

#[test]
fn count_by_type_one_of_each() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
        Box::new(Rectangle {
            width: 3.0,
            height: 4.0,
        }),
    ];
    let counts = count_by_type(&shapes);
    assert_eq!(counts.get("Circle"), Some(&1));
    assert_eq!(counts.get("Square"), Some(&1));
    assert_eq!(counts.get("Rectangle"), Some(&1));
    assert_eq!(counts.len(), 3);
}

#[test]
fn count_by_type_multiple_circles_only() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Circle { radius: 2.0 }),
        Box::new(Circle { radius: 3.0 }),
    ];
    let counts = count_by_type(&shapes);
    assert_eq!(counts.get("Circle"), Some(&3));
    assert_eq!(counts.get("Square"), None);
    assert_eq!(counts.get("Rectangle"), None);
    assert_eq!(counts.len(), 1);
}

#[test]
fn count_by_type_mixed_counts() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 1.0 }),
        Box::new(Square { side: 2.0 }),
        Box::new(Rectangle {
            width: 1.0,
            height: 1.0,
        }),
        Box::new(Circle { radius: 2.0 }),
    ];
    let counts = count_by_type(&shapes);
    assert_eq!(counts.get("Circle"), Some(&2));
    assert_eq!(counts.get("Square"), Some(&2));
    assert_eq!(counts.get("Rectangle"), Some(&1));
}

#[test]
fn shape_area_computations_via_dynamic_dispatch() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Square { side: 3.0 }),
        Box::new(Rectangle {
            width: 4.0,
            height: 5.0,
        }),
    ];
    let areas: Vec<f64> = shapes.iter().map(|s| s.area()).collect();
    assert!((areas[0] - std::f64::consts::PI * 4.0).abs() < 1e-9);
    assert_eq!(areas[1], 9.0);
    assert_eq!(areas[2], 20.0);
}

// --- Notifier decorator chain -------------------------------------------------------------

#[test]
fn email_notifier_sends_one_message() {
    let n = EmailNotifier;
    assert_eq!(n.send("hi"), vec!["email: hi".to_string()]);
}

#[test]
fn sms_decorator_sends_sms_then_inner() {
    let n = SmsDecorator {
        inner: Box::new(EmailNotifier),
    };
    assert_eq!(
        n.send("hi"),
        vec!["sms: hi".to_string(), "email: hi".to_string()]
    );
}

#[test]
fn logging_decorator_sends_log_then_inner() {
    let n = LoggingDecorator {
        inner: Box::new(EmailNotifier),
    };
    assert_eq!(
        n.send("hi"),
        vec!["log: hi".to_string(), "email: hi".to_string()]
    );
}

#[test]
fn nested_decorators_log_then_sms_then_email() {
    let n = LoggingDecorator {
        inner: Box::new(SmsDecorator {
            inner: Box::new(EmailNotifier),
        }),
    };
    assert_eq!(
        n.send("hi"),
        vec![
            "log: hi".to_string(),
            "sms: hi".to_string(),
            "email: hi".to_string(),
        ]
    );
}

#[test]
fn nested_decorators_different_order_changes_output_order() {
    let n = SmsDecorator {
        inner: Box::new(LoggingDecorator {
            inner: Box::new(EmailNotifier),
        }),
    };
    assert_eq!(
        n.send("hi"),
        vec![
            "sms: hi".to_string(),
            "log: hi".to_string(),
            "email: hi".to_string(),
        ]
    );
}

#[test]
fn decorator_chain_uses_the_actual_message() {
    let n = SmsDecorator {
        inner: Box::new(EmailNotifier),
    };
    assert_eq!(
        n.send("urgent"),
        vec!["sms: urgent".to_string(), "email: urgent".to_string()]
    );
}
