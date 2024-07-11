import { test } from "@playwright/test";
import { shuffle } from "lodash";

const PIECES = ['P', 'K', 'B', 'R', 'Q', 'K'];
const TEAM_BLURB_REGEX = /You are playing (.*)\./gi;

test('emulate player', async ({ page }: any) => {
  test.slow();
  await page.goto('https://wildchess.dev');

  const button = page.getByRole('button', { name: 'Play' });
  console.log(await button.count());
  await button.click();

  await page.getByTestId("chessboard").waitFor();
  const board = page.locator("[data-testid=\"chessboard\"]");

  const teamText = await page.getByText(TEAM_BLURB_REGEX).last().textContent();
  const myTeam = (Array.from(teamText.matchAll(TEAM_BLURB_REGEX)) as string[][])[0][1][0].toLowerCase();

  while(true) {
    const myTurnLocator = page.getByText("MY TURN").last();
    findMove: while (await myTurnLocator.isVisible()) {
      for (const piece of shuffle(PIECES)) {
        for (const pieceLocator of shuffle(await board.locator(`[data-piece="${myTeam}${piece}"]`).all())) {
          const startSquare = await pieceLocator.getAttribute('data-square');
          await pieceLocator.click();
          const targetLocator = board.locator(`[data-gamestate="target"]`).first();
          const targetSquare = await targetLocator.getAttribute('data-square');
          const targetExists = await targetLocator.isVisible();
          console.log(targetExists + ": " + startSquare + " " + targetSquare);
          if (targetExists) {
            await pieceLocator.dragTo(targetLocator);
            await board.locator(`[data-piece="${myTeam}${piece}"][data-square="${targetSquare}"]`).waitFor();
            break findMove;
          }
        }
      }
    }
    await myTurnLocator.waitFor(10000);
  }
});
