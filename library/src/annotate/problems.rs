use super::annotations::*;

use {depiction::*, problemo::*};

//
// DepictProblem
//

/// Depict problem.
pub trait DepictProblem {
    /// Depict problem.
    fn print_default_depiction(&self);
}

impl DepictProblem for Problem {
    fn print_default_depiction(&self) {
        if let Some(cause) = self.cause_of_type::<DepictRef>() {
            cause.error.print_default_depiction();

            // if let Some(invalid_key) = depict.downcast_ref::<InvalidKeyError>() {
            //     println!("3333 INVALID KEY: {}", invalid_key.key);
            // }

            if let Some(annotations) = cause.attachment_of_type::<Annotations>() {
                annotations.print_default_depiction();
            }
        } else {
            println!("{}", self);
        }
    }
}

//
// DepictProblems
//

/// Depict problems.
pub trait DepictProblems {
    /// Depict problems.
    fn print_default_depiction(&self);
}

impl DepictProblem for Problems {
    fn print_default_depiction(&self) {
        for problem in self {
            problem.print_default_depiction();
        }
    }
}
