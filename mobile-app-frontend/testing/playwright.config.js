import { defineConfig, devices } from '@playwright/test'; // import devices

const FRONTEND_SERVER = 'http://0.0.0.0:8080';

export default defineConfig({
	fullyParallel: false,
	use: {
		// Base URL to use in actions like `await page.goto('/')`.
		baseURL: FRONTEND_SERVER,
		// Collect trace when retrying the failed test.
		// trace: 'on-first-retry',
	},
	projects: [
		{ name: 'setup', testMatch: /.*\.setup\.js/ },
		{
			name: 'Mobile',
			use: {
				...devices['Pixel 5'],
				viewport: { width: 640, height: 920 },
				storageState: 'playwright/.auth/user.json',
			},
			dependencies: ['setup'],
		},
	],
});