import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Soroban Cookbook',
  tagline: 'A comprehensive guide to building smart contracts on Stellar with Soroban',
  favicon: 'img/logo.svg',

  future: {
    v4: true,
  },

  url: 'https://soroban-cookbook.dev',
  baseUrl: '/',

  organizationName: 'Soroban-Cookbook',
  projectName: 'Soroban_Cookbook_online',

  customFields: {
    /** POST endpoint accepting JSON `{ "email": string }`. Set via env at build time for real integrations. */
    newsletterEndpoint: process.env.NEWSLETTER_ENDPOINT ?? '',
    /** Soroban Cookbook Discord invite link. Set DISCORD_INVITE_URL at build time once the server is created. */
    discordInviteUrl: process.env.DISCORD_INVITE_URL ?? '',
  },

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  // Theme initialization script to prevent flash of incorrect theme
  scripts: [
    {
      src: '/js/themeInit.js',
      async: false,
    },
  ],

  // Meta tags for theme color + social previews (see CONTRIBUTING — SEO & social metadata)
  headTags: [
    {
      tagName: 'meta',
      attributes: {
        name: 'theme-color',
        content: '#1e1e2e',
      },
    },
    // Preload the Inter variable font (latin woff2) — critical for above-the-fold text.
    // The href must match the path emitted by @fontsource-variable/inter after build.
    {
      tagName: 'link',
      attributes: {
        rel: 'preload',
        href: '/assets/fonts/inter-latin-wght-normal.woff2',
        as: 'font',
        type: 'font/woff2',
        crossorigin: 'anonymous',
      },
    },
    // Preload JetBrains Mono for code blocks.
    {
      tagName: 'link',
      attributes: {
        rel: 'preload',
        href: '/assets/fonts/jetbrains-mono-latin-wght-normal.woff2',
        as: 'font',
        type: 'font/woff2',
        crossorigin: 'anonymous',
      },
    },
    {
      tagName: 'meta',
      attributes: {
        property: 'og:type',
        content: 'website',
      },
    },
    {
      tagName: 'meta',
      attributes: {
        property: 'og:site_name',
        content: 'Soroban Cookbook',
      },
    },
    {
      tagName: 'meta',
      attributes: {
        property: 'og:image',
        content: 'https://soroban-cookbook.dev/img/soroban-social-card.png',
      },
    },
    {
      tagName: 'meta',
      attributes: {
        property: 'og:image:width',
        content: '1200',
      },
    },
    {
      tagName: 'meta',
      attributes: {
        property: 'og:image:height',
        content: '630',
      },
    },
    {
      tagName: 'meta',
      attributes: {
        name: 'twitter:card',
        content: 'summary_large_image',
      },
    },
    {
      tagName: 'meta',
      attributes: {
        name: 'twitter:image',
        content: 'https://soroban-cookbook.dev/img/soroban-social-card.png',
      },
    },
  ],

  plugins: [
    [
      require.resolve('@easyops-cn/docusaurus-search-local'),
      {
        hashed: true,
        language: ['en'],
        highlightSearchTermsOnTargetPage: true,
        explicitSearchResultPath: true,
        indexDocs: true,
        indexPages: true,
        indexBlog: false,
      },
    ],
  ],

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          routeBasePath: '/docs',
          editUrl: 'https://github.com/Soroban-Cookbook/Soroban_Cookbook_online/tree/main/documentation/',
        },
        blog: false,
        theme: {
          customCss: [
            './src/css/fonts.css',
            './src/css/design-tokens.css',
            './src/css/breakpoints.css',
            './src/css/badges-tags.css',
            './src/css/custom.css',
            './src/css/search-experience.css',
          ]
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: 'img/soroban-social-card.png',
    colorMode: {
      defaultMode: 'dark',
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: 'Soroban Cookbook',
      logo: {
        alt: 'Soroban Logo',
        src: 'img/logo.svg',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'tutorialSidebar',
          position: 'left',
          label: 'Docs',
        },
        {
          href: process.env.DISCORD_INVITE_URL ?? 'https://discord.gg/YNBu3jKEF',
          label: 'Discord',
          position: 'right',
        },
        {
          href: 'https://github.com/Soroban-Cookbook/Soroban_Cookbook_online',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Learn',
          items: [
            {
              label: 'Documentation',
              to: '/',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'Soroban Cookbook Discord',
              href: process.env.DISCORD_INVITE_URL ?? 'https://discord.gg/YNBu3jKEF',
            },
            {
              label: 'Stellar Discord',
              href: 'https://discord.gg/YNBu3jKEF',
            },
            {
              label: 'Stack Overflow',
              href: 'https://stackoverflow.com/questions/tagged/soroban',
            },
            {
              label: 'Code of Conduct',
              href: 'https://github.com/Soroban-Cookbook/Soroban_Cookbook_online/blob/main/CODE_OF_CONDUCT.md',
            },
          ],
        },
        {
          title: 'Resources',
          items: [
            {
              label: 'Soroban Docs',
              href: 'https://developers.stellar.org/docs/smart-contracts',
            },
            {
              label: 'GitHub',
              href: 'https://github.com/Soroban-Cookbook/Soroban_Cookbook_online',
            },
          ],
        },
      ],
      copyright: `Built by the community • Powered by Stellar • MIT License • © ${new Date().getFullYear()}`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.vsDark,
      additionalLanguages: ['rust', 'toml', 'bash'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
