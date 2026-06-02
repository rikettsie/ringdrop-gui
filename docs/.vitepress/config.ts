import { defineConfig } from "vitepress";

export default defineConfig({
  title: "ringdrop",
  description: "P2P file sharing with ring-based access control",
  cleanUrls: true,

  head: [["link", { rel: "icon", href: "/favicon.png" }]],

  themeConfig: {
    logo: "/favicon.png",
    siteTitle: "ringdrop",

    nav: [
      { text: "Install", link: "/install" },
      { text: "Guide", link: "/quickstart" },
      {
        text: "GitHub",
        link: "https://github.com/rikettsie/ringdrop-gui",
      },
    ],

    sidebar: [
      {
        text: "Getting started",
        items: [
          { text: "Install", link: "/install" },
          { text: "Quickstart", link: "/quickstart" },
        ],
      },
      {
        text: "Features",
        items: [
          { text: "Blobs", link: "/guide/blobs" },
          { text: "Rings & access control", link: "/guide/rings" },
          { text: "Peers", link: "/guide/peers" },
          { text: "Grants", link: "/guide/grants" },
          { text: "Remote catalog", link: "/guide/remote" },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/rikettsie/ringdrop-gui" },
    ],

    footer: {
      message: "Released under the MIT License.",
    },

    search: { provider: "local" },
  },
});
