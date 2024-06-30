import { test, expect } from "@playwright/test";

test("homepage has title and nav brand", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Flow Car Studio");

  await expect(page.locator("a.navbar-brand")).toHaveText("Flow Car Studio");
});
