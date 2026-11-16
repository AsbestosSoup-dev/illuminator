# Notes

Short design decisions and measurements. Future-you will thank present-you.

## Decisions

<!-- M3: enum vs Box<dyn Hittable>? Write 2-3 lines on what you picked and why. -->

## Timings

Record every render so each optimisation has a baseline.
Use the same scene and settings within a row group.

| Milestone | Scene | Resolution | Samples | Threads | Time |
|-----------|-------|------------|---------|---------|------|
| M7 | random spheres | 400x225 | 50 | 1 | |
| M8 | random spheres | 400x225 | 50 | all | |
| M9 | mesh | | | all | |
| M10 | mesh + BVH | | | all | |
