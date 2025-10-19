# iced-components-test

Based on the guide here:
- https://jl710.github.io/iced-guide/app_structure/full-code.html

Architecture:
- Components return Actions.
- Actions are eithiers tasks passed upwards, or things that the parent must do.
- update() takes in the message for the current component, and passes it down to the relevant child.
