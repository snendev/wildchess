import { JSX } from "preact";

interface PromotionPiecesProps {
  icons: string[] | null
  selectIcon: (index: number) => void
}

export default function PromotionPieces({icons, selectIcon}: PromotionPiecesProps): JSX.Element {
  return (
    <div class="flex flex-col w-16">
      {icons?.map((icon, index) => {
        const blob = new Blob([icon], {type: 'image/svg+xml'});
        const url = URL.createObjectURL(blob);
        return (
          <button onClick={() => selectIcon(index)}>
            <img src={url} class="w-16" />
          </button>
        )
      })}
    </div>
  )
}
