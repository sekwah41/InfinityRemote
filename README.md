# Infinity Remote

I've only designed this as an app to practice making cross-platform projects as well as create a remote for my own personal use.

PR's are welcome and encouraged.

If this project gets any attention then I will look to add guides on how to contribute.

## Can you support X device?
If the device has documentation on how to communicate with it then I can possibly have a crack at it,
though unless I own the device I will likely not be able to support it.

This is one of the reasons I want to make this open source. 
If you want to add support for a device you own then you can.

## What is the motivation behind this project?
My father used to have an app which could do some of this, though it is long since dead and I wanted an excuse to publish something.
Also, ive always enjoyed helping run & create open source projects and this is a good excuse to get back into it.

While most if not all of this could be probably organised or set up using a project such as [Home Assistant](https://www.home-assistant.io/),
I wanted to make an app that would solely work on your phone and not require a server to be running.

Though if you are into home automation and want a much wider range of devices supported I would suggest checking that out.

## Unique points you plan?
I want to make this app as simple as possible to use and set up.
I also want to make it as easy as possible to add new devices to the app, e.g. sharing a remote for guests or family members to use.

## How do I build this project?
TODO 

## Coding Style/Best Practices
I am new to rust, so I am still learning the best practices and style of the language.
If you have any suggestions feel free to open a PR or Issue and either link to examples or explain why.

## Planned Devices

## Why Tauri?

I'll be honest I just currently have an interest in rust atm.

Another reason is purely how safe it seems to be writing in rust and portable
e.g. you can compile to Apple Watches https://doc.rust-lang.org/rustc/platform-support/apple-watchos.html 
or WearOs and have a remote on your wrist using the same main underlying codebase for device communication.

Also given the rust backend and WebUI frontend, it could also be possible to run this on a headless server as a webapp removing the need for install.
This would allow an NFC tag or QR code to just be placed in a room with the same Wi-Fi for connection.

Though the focus is a mobile and desktop app for now.

# Setting up the project
If you want to just dev and test on PC or Mac then its a lot easier, though make sure to download the prerequisites for the platform you are on.
https://beta.tauri.app/guides/prerequisites/
