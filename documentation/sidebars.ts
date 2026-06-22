import type { SidebarsConfig } from '@docusaurus/plugin-content-docs';

/**
 * Soroban Cookbook Sidebar Configuration
 * Creating a structured learning path for Soroban development
 */
const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    {
      type: 'category',
      label: 'Getting Started',
      items: [
        'getting-started/setup',
        'getting-started/setup-linux',
        'getting-started/setup-windows',
        'getting-started/first-contract',
        'getting-started/building-and-compilation',
        'getting-started/deploy-testnet',
        'getting-started/deploy-mainnet',
        'getting-started/contract-interaction',
        'contributing',
        'contributing/add-tested-example',
      ],
    },
    {
      type: 'category',
      label: 'Core Concepts',
      items: [
        'concepts/introduction',
        'concepts/overview',
        'concepts/best-practices',
        'concepts/storage',
        'concepts/authorization',
        'concepts/events',
        'concepts/gas-and-resources',
        'concepts/cross-contract-invocation',
      ],
    },
    {
      type: 'category',
      label: 'Patterns',
      items: [
        'patterns/overview',
        'patterns/hello-world',
        'patterns/custom-types',
        'patterns/authorization',
        'patterns/optimization-playbook',
        'patterns/lifecycle-upgrades',
      ],
    },
    {
      type: 'category',
      label: 'Security',
      items: [
        'security/fundamentals',
      ],
    },
    {
      type: 'category',
      label: 'Design System',
      items: [
        'design-system/buttons',
        'design-system/typography',
        'design-system/badges-tags',
        'design-system/empty-states',
      ],
    },
    {
      type: 'category',
      label: 'Components',
      items: [
        'components/buttons',
        'components/testimonials',
      ],
    },
    {
      type: 'category',
      label: 'Responsive',
      items: [
        'responsive/breakpoints',
      ],
    },
  ],
};

export default sidebars;
