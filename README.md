# configger

configger wants to be a scalable, versatile and light weight configuration file manager for linux

  - track configuration files for changes
  - differential backups with version control and history
  - filesets and taxonomy
  - migration tools for copying over configuration files to other servers
  - remote monitoring and rapid deployment

### Current status

configger has barely got off the ground. There's a working file tracker, and a command line interface for adding and removing files from the tracker. I'm an ambitious person.

### Help me test

Clone into the repository.

Setup:

```sh
$ chmod +x setup.sh
$ ./setup.sh
$ ./configger-cli.py --help
$ ./configger-cli.py add file1 file2 file3 ...
```
Run the tracker:

```sh
$ python3 tracker.py
```

The tracker triggers an event on IN_CLOSE_WRITE. Currently, the eventhandler prints to stdout.

### Todos

 - tracker as a class
 - run as a service
 - implement tracker-cli to manage service
 - implement autobackup

License
----
MIT