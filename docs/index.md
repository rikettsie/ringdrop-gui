---
layout: home

hero:
  name: ringdrop
  text: P2P file sharing with rings
  tagline: >
    Share files directly with your circles.
    Control who sees what — no cloud required.
  actions:
    - theme: brand
      text: Install
      link: /install
    - theme: alt
      text: Quickstart
      link: /quickstart

features:
  - icon: 🔒
    title: Ring-based access control
    details: >
      Group peers into named rings. A blob is only served to peers
      that belong to a ring with Read permission on it. Revoke access
      by removing a peer from the ring — no keys to rotate.

  - icon: ⚡
    title: Direct P2P, relay fallback
    details: >
      Built on QUIC with NAT hole-punching. Transfers go peer-to-peer
      when possible; a relay handles NAT-blocked paths automatically.
      No central server stores your files.

  - icon: 🖥️
    title: Desktop GUI
    details: >
      Native desktop app for Linux, macOS, and Windows.
      Connects to a local ringdrop daemon over IPC — the same daemon
      the CLI uses, sharing all your data and settings.

  - icon: 📋
    title: Remote catalog
    details: >
      Browse what a peer is sharing with you before downloading.
      Each ring exposes a curated list of blobs to its members.
      Pull only what you want with live progress tracking.
---
