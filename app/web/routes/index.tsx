import { useSignal } from "@preact/signals";

import Page from "../components/Page.tsx";

export default function Home() {
    const count = useSignal(3);
    return (
        <Page>
            <h1 class="text-4xl font-bold mb-12">Wild Chess</h1>
            <a href="/chess">Chess</a>
        </Page>
    );
}
