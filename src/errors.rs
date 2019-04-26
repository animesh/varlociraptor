use custom_error::custom_error;

custom_error! {pub TestcaseError
    MissingCandidates = "candidate variants must be provided via --candidates",
    MissingPrefix = "testcase prefix must be given with --testcase-prefix",
    NoCandidateFound = "no candidate variant at the given locus",
    InvalidLocus = "invalid locus for --testcase-locus. Use CHROM:POS syntax",
    InvalidIndex = "invalid variant index given, must be not higher than the number of variants at the locus",
}

custom_error! {pub CallCNVError
    InvalidMinBayesFactor = "invalid minimum bayes factor, must be > 1.0",
}
