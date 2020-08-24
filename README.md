# Rust Stackmachine

Intended to be used for training material for interns at Pacific Northwest National Laboratory.
Points of interest include the `fork` command, which creates a new thread and copies data over to the thread from the original stackmachine.
This will serve as introductory material to fork/join methodologies in the HPC world, but approachable enough that the interns will be able to hack on the program to alter its behavior and get a feel for how an actual job scheduler will deal with a parallel program.

[![Build Status](https://travis-ci.org/ashermancinelli/stackmachine.svg?branch=master)](https://travis-ci.org/ashermancinelli/stackmachine)

Please see the `examples` directory for code that will run on the stackmachine.
The stackmachine has a global stack which can be pushed to and popped from.

