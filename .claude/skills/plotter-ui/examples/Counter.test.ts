import { expect, test } from "@rstest/core";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";

import Counter from "./Counter.svelte";

test("Counter", async () => {
  const user = userEvent.setup();
  render(Counter);

  const button = screen.getByRole("button");
  expect(button).toHaveTextContent(`clicks: 0`);

  await user.click(button);
  expect(button).toHaveTextContent(`clicks: 1`);
});
