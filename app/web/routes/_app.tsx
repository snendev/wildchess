import { type PageProps } from "$fresh/server.ts";
export default function App({ Component }: PageProps) {
    return (
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Wild Chess</title>
                {/* jquery */}
                <script src="https://code.jquery.com/jquery-3.5.1.min.js"
                    integrity="sha384-ZvpUoO/+PpLXR1lu4jmpXWu80pZlYUAfxl5NsBMWOEPSjUn/6Z/hRTt8+pR6L4N2"
                    crossorigin="anonymous"></script>
                {/* chessboardjs styles */}
                <link rel="stylesheet"
                    href="https://unpkg.com/@chrisoakman/chessboardjs@1.0.0/dist/chessboard-1.0.0.min.css"
                    integrity="sha384-q94+BZtLrkL1/ohfjR8c6L+A6qzNH9R2hBLwyoAfu3i/WCvQjzL2RQJ3uNHDISdU"
                    crossorigin="anonymous" />
                <script src="https://unpkg.com/@chrisoakman/chessboardjs@1.0.0/dist/chessboard-1.0.0.min.js"
                    integrity="sha384-8Vi8VHwn3vjQ9eUHUxex3JSN/NFqUg3QbPyX8kWyb93+8AC/pPWTzj+nHtbC5bxD"
                    crossorigin="anonymous"></script>
                {/* custom styles */}
                <link rel="stylesheet" href="/styles.css" />
            </head>
            <body>
                <Component />
            </body>
        </html >
    );
}
