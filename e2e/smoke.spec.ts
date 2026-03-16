import { test, expect } from "@playwright/test";

test("打开应用欢迎页", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByText("欢迎使用 AI Novel Studio")).toBeVisible();
});

