import { VNode } from "preact/hooks";

export default function Page({ children }: { children: VNode }) {
    return (
        <div class="px-4 py-8 mx-auto bg-[#86efac] h-screen">
            <div class="max-w-screen-md mx-auto flex flex-col items-center justify-center">
                {children}
            </div>
        </div>
    );
}
