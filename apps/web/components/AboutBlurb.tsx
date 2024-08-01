import { JSX } from "preact";

export default function AboutBlurb(): JSX.Element {
  return (
    <div class="p-4 border-2 border-black bg-[#cadfdf]">
      <h3 class="text-2xl">About WildChess</h3>
      <br />
      <p>
        If you want to notified about updates to the site, please sign up for our{' '}
        <a>mailing list</a>.
      </p>
      <br />
      <p>
        WildChess is about playing Chess in a new and unexplored way.

        Every [<b>cough ahem</b>] we create a new set of variant positions intended to feel similar to chess
        in principle, while leading to vastly different positions and strategies.
      </p>
      <br />
      <p>
        This site is in an ALPHA stage, so its features are under active development and will not be stable.
        
        Additionally, there are a lot of fun new features on the way!
      </p>
      <br />
      <p>
        This website may not be functional on all browsers! If your browser crashes, please try loading in Chrome.
      </p>
      <br />
      <p>
        In the future, expect the following features:
      </p>
      <ul class="p-2 list-disc list-inside">
        <li>
          A play setting that provides a unique, randomized board each game
        </li>
        <li>
          Settings for different board sizes and styles (such as including rules from other chess variants)
        </li>
        <li>More time controls</li>
        <li>ELO-based matchmaking</li>
        <li>More board features (arrows, premoves etc.)</li>
      </ul>
      <p>
        If you have any feedback, please provide it in the{' '}
        <a>Bevy Discord</a>.
      </p>
    </div>
  )
}
