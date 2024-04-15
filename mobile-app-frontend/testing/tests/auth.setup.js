import { test as setup, expect } from '@playwright/test';

const authFile = 'playwright/.auth/user.json';

setup('authenticate', async ({ page }) => {
    await page.goto('/');
	await page.locator('a[href="/auth"]').click();
	await page.getByRole('button', { name: 'Войти' }).click();
	await page.waitForURL('/new-tasks');
	await page.locator('a[href="/auth"]').click();
	await expect(page.locator('.heading')).toContainText('сессия');

    await page.context().storageState({ path: authFile });
});