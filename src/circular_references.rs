use std::cell::{RefCell};
use std::rc::Rc;

pub fn circular_references_func() {
    struct Student<'a> {
        name: String,
        courses: Vec<&'a Course<'a>>
    }

    impl<'a> Student<'a> {
        fn new(name: &str) -> Student<'a> {
            Student { name: name.into(), courses: Vec::new()}
        }
    }

    struct Course<'a> {
        name: String,
        students: Vec<&'a Student<'a>>
    }

    impl<'a> Course<'a> {
        fn new(name: &str) -> Course<'a> {
            Course {
                name: name.into(),
                students: Vec::new()
            }
        }

        fn add_student(&'a mut self, student: &'a mut Student<'a>) {
            // student.courses.push(self);
            // self.students.push(student);
            // student.courses.push(self);
            // |             --------------------------
            //     |             |                    |
            //     |             |                    immutable borrow occurs here
            //     |             argument requires that `*self` is borrowed for `'a`
            // 28 |             self.students.push(student);
            // |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
        }
    }

    // let mut john = Student:: new("John");
    // let mut course = Course::new("Rust Course");
    // course.add_student(&mut john);

    // Let's have a fix below with RefCell and Rc
    circular_fix();
}

fn circular_fix() {

    struct Platform<'a> {
        enrollments: Vec<Enrollment<'a>>
    }

    impl<'a> Platform<'a> {
        fn new() -> Platform<'a> {
            Platform {
                enrollments: Vec::new()
            }
        }

        fn enroll(&mut self, student: &'a Student, course: &'a Course) {
            self.enrollments.push(
                Enrollment::new(student,course)
            )
        }
    }
    struct Enrollment<'a> {
        student: &'a Student,
        course: &'a Course
    }

    impl<'a> Enrollment<'a> {
        fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
            Enrollment { student, course }
        }
    }
    struct Student {
        name: String,
        courses: Vec<Rc<RefCell<Course>>>
    }

    impl Student {
        fn courses(&self, platform: Platform) -> Vec<String> {
            platform.enrollments.iter().filter(
                |&e| e.student.name == self.name
            ).map(|e| e.course.name.clone()).collect()
        }
    }
    impl Student {
        fn new(name: &str) -> Student {
            Student { name: name.into(), courses: Vec::new()}
        }
    }

    struct Course {
        name: String,
        students: Vec<Rc<RefCell<Student>>>
    }

    impl Course  {
        fn new(name: &str) -> Course {
            Course {
                name: name.into(),
                students: Vec::new()
            }
        }


        fn add_student(
            course: Rc<RefCell<Course>>,
            student: Rc<RefCell<Student>>
        ) {
            student.borrow_mut().courses.push(course.clone());
            course.borrow_mut().students.push(student);
        }
    }

    let john =  Rc::new(
        RefCell::new(
            Student::new("John")
        )
    );
    let course= Course::new("Rust Course");
    let magic_course = Rc::new(
        RefCell::new(course)
    );
    Course::add_student(magic_course, john);
}


fn better_design() {

    struct Platform<'a> {
        enrollments: Vec<Enrollment<'a>>
    }

    impl<'a> Platform<'a> {
        fn new() -> Platform<'a> {
            Platform {
                enrollments: Vec::new()
            }
        }

        fn enroll(&mut self, student: &'a Student, course: &'a Course) {
            self.enrollments.push(
                Enrollment::new(student,course)
            )
        }
    }

    struct Student {
        name: String
    }

    impl Student {
        fn courses(&self, platform: Platform) -> Vec<String> {
            platform.enrollments.iter().filter(
                |&e| e.student.name == self.name
            ).map(|e| e.course.name.clone()).collect()
        }
    }

    struct Course {
        name: String,
    }

    struct Enrollment<'a> {
        student: &'a Student,
        course: &'a Course
    }

    impl<'a> Enrollment<'a> {
        fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
            Enrollment { student, course }
        }
    }

    let jane = Student {
        name: "John".into()
    };

    let cours = Course {
        name: "Into to Rust".into()
    };

    let mut p = Platform::new();
    p.enroll(&jane, &cours);

    for c in jane.courses(p) {
        println!("John is taking {}", c)
    }
}
