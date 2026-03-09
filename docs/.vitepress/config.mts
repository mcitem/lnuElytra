import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "lnuElytra",
  description: "LNU 岭南师范学院 正方教务系统 抢课 选课工具",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "指南", link: "/tutorial/what-is-lnuElytra" },
      {
        text: "API参考",
        link: "/tutorial/interface",
      },
    ],
    sidebar: [
      {
        text: "简介",
        items: [
          { text: "什么是 lnuElytra?", link: "/tutorial/what-is-lnuElytra" },
          { text: "教学班", link: "/tutorial/jxb" },
        ],
      },
      {
        text: "使用教程",
        items: [
          {
            text: "Rust（推荐）",
            items: [{ text: "快速入门", link: "/tutorial/rust" }],
          },
          {
            text: "Python",
            items: [{ text: "快速入门", link: "/tutorial/python" }],
          },
          {
            text: "API参考",
            link: "/tutorial/interface",
          },
        ],
      },
      {
        text: "快速访问",
        items: [
          {
            text: "Crates.io",
            link: "https://crates.io/crates/lnu-elytra",
          },
          {
            text: "Docs.rs",
            link: "https://docs.rs/lnu-elytra",
          },
          { text: "PyPI", link: "https://pypi.org/project/lnu-elytra" },
          { text: "GitHub", link: "https://github.com/mcitem/lnuElytra" },
        ],
      },
    ],
    notFound: {
      link: "/tutorial",
    },
    search: {
      provider: "local",
    },
    socialLinks: [
      { icon: "github", link: "https://github.com/mcitem/lnuElytra" },
      { icon: "createio", link: "https://crates.io/crates/lnu-elytra" },
    ],
    footer: {
      message: "AGPL-3.0 License",
      copyright: "Copyright © 2026-present github.com/mcitem",
    },
    editLink: {
      pattern: "https://github.com/mcitem/lnuElytra/edit/main/docs/:path",
      text: "在 GitHub 中编辑本页",
    },
    lastUpdated: {
      text: "最后更新于",
    },
  },
  cleanUrls: true,
  sitemap: {
    hostname: "https://lnu-elytra.mcitem.net",
  },
});
