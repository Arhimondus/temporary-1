import { test, expect, request } from '@playwright/test';
import axios from 'axios';

const BACKEND_SERVER = 'http://127.0.0.1:9090';

// function frontend(path = '/') {
//	return `${FRONTEND_SERVER}${path}`;
// }

function backend(path = '/') {
	return `${BACKEND_SERVER}${path}`;
}

// test('auth', async ({ page }) => {
// 	await page.goto('/');
// 	await page.locator('a[href="/auth"]').click();
// 	await page.getByRole('button', { name: 'Войти' }).click();
// 	await page.waitForURL('/new-tasks');
// 	await page.locator('a[href="/auth"]').click();
// 	await expect(page.locator('.heading')).toContainText('сессия');
// 	// await expect(page).toHaveTitle(/Playwright/);
// });

// await page.locator('a[href="/auth"]').click();
// await page.getByRole('button', { name: 'Выход' }).click();
// await page.getByRole('button', { name: 'Выдать обычную задачу' }).click();
// await page.getByRole('button', { name: 'Выдать нормировочную задачу' }).click();
// await page.getByRole('button', { name: 'Выдать смешанную задачу' }).click();

// await page.getByRole('button', { name: 'Продолжить' }).click();
// await page.getByRole('button', { name: 'Пауза' }).click();
// await page.getByRole('button', { name: 'Принятые' }).click();
// await page.getByRole('button', { name: 'Завершённые' }).click();

// await page.getByRole('button', { name: 'Текущая' }).click();


// await page.getByRole('button', { name: 'Задача' }).click();
// await page.getByRole('button', { name: 'Инструменты' }).click();

test('task_fg', async ({ page }) => {
	await page.goto('/auth');
	// await axios.post(backend('/add-test-task/fg'));
	console.log(backend('/add-test-task/fg'));
	await axios.post(backend('/add-test-task/fg'));
	await page.locator('a[href="/new-tasks"]').click();
	await expect(page).toHaveURL('/new-tasks');
});