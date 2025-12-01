# Architecture Overview

Nomad consists of three main layers

## 1. Nomad Core

The core logic of Nomad, it provides the Component Bus, IPC, etc. It is Completely `no_std`.
This is analgous to cFE in NASA cFS.

## 2. Nomad OSAL (Operating System Abstraction Layer)

The OS integration layer, provides wrapper to OS provides resources such as threading,
syncronization resources, filesystems, and implements traits that Nomad might expose that
require the usage of a operating system.

Currently two OS's are supported

- POSIX
- "baremetal" (RTIC)(TODO in the future)

This can be choosen via Rust crate features

## 3. HAL

Platform and Hardware specific things. This breakouts things such as GPIO, etc. This
contains anything that is board specific. HAL can be used to implement platform specific features
or platform agnostic features. For example OSAL could expose GPIO functionality to FSW or core,
which OSAL might internally implement using HAL. Or maybe platform exposes somethign that isnt agnostic
that you would like to tap into

## Components

Utilizing the three layers above, you can build your own FSW Components. FSW Components are independent 
FSW modules that can be tailored to whatever you might need, they are analgous to services/microservices. 

You can utilize the component bus is nomad core to register and "wire up" components, allowing you to create
your own FSW

## FSW Application

You can implement your own Flight Software Application by creating your own components and bringing them together
on the component bus.

Nomad provides a standard collection of components in `nomad-core` which user can utilize in their own FSW. The project
provides a reference flight software called `nomad-fsw`, this is a reference FSW originally meant for the Laika board. 
It provides the bare minimum flight software platform to get up and running with all the components provided in `nomad-core`.
nomad-fsw is enough to get up and ready for most projects, so users can instead just extend nomad-fsw with their own components.
