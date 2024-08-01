import { JSX, VNode } from "preact/hooks";

interface ChildrenProps {
  children: VNode
}

export default function Page({ children }: ChildrenProps): JSX.Element {
  return (
    <Background>
      <Header />
      <Body>
        {children}
      </Body>
    </Background>
  );
}

function Background({ children }: ChildrenProps): JSX.Element {
  return (
    <div class="bg-[#e2fff2] h-full min-h-screen">
      {children}
    </div>
  )
}

function Header(): JSX.Element {
  return (
    <div class="border-b-2 border-black">
      <div class="max-w-screen-lg h-[60px] m-auto flex flex-row justify-between items-center bg-[#e2fff2]">
        <h1 class="text-2xl">WildChess</h1>
      </div>
    </div>
  );
}

function Body({ children }: ChildrenProps): JSX.Element {
  return (
    <div class="max-w-screen-lg mx-auto px-2 py-6 flex flex-col items-center justify-center">
      {children}
    </div>
  )
}
