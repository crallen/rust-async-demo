# Async Workers Demo in Rust

This little demo started as part of another project I've been working on. I
needed to run a fixed size pool of workers to generate sample data. I'm still
relatively new to Rust, so after fumbling with this for a bit I decided to pull
it out into its own project and get the workers running in isolation.

The project is nothing fancy. It starts up 3 workers that sleep for a random
amount of time between 2 and 5 seconds before printing to the console. It will
continue to run until interrupted (i.e. Ctrl+C). It is designed to gracefully
shut down when the interrupt is received.