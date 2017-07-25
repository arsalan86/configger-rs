# configger [![Build Status](https://travis-ci.org/arsalan86/configger-rs.svg?branch=master)](https://travis-ci.org/arsalan86/configger-rs)

configger wants to be a scalable, versatile and light weight configuration file manager for linux, written in rust

  - track configuration files for changes
  - differential backups with version control and history
  - filesets and taxonomy
  - migration tools for copying over configuration files to other servers
  - remote monitoring and rapid deployment

### Current status

configger has barely got off the ground. There's a working file tracker, and a (not yet implemented) command line interface for adding and removing files from the tracker. I'm an ambitious person.

### Todos

 - run as a service
 - run in threaded mode
 - implement cli
 - implement autobackup

License
----
MIT License