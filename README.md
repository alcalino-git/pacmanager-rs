# Pacmanager-rs

A full rewrite of my [pacmanager](https://github.com/alcalino-git/pacmanager) project, this time in rust 

Unlike the original C++ project, this one can be considered production-ready

### Description
Pacmanager is a simple GUI wrapper for the pacman package manager intended for use on Arch or Arch-based systems

### Capabilities
- Search for packages on the remote repos
- Filter packages by status
- Sort packages by their properties
- View info about packages
- Install and update packages
- Perform full system updates
- Remove local packages
- Ability to display package installation progress in the UI

### Improvements
Thanks to rust's memory safety, this version features no crashes or segmentation faults. The search feature is also faster, the UI is more responsive and the overall experience is better.
