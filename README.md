# LinkedIn Carousel Generator
<p align="center">
 <img src="https://github.com/mdabir1203/JohnCrickettChallenge/assets/66947064/d5bebcb0-4d38-4214-90dc-350a3aa4c441" width="350" height="350">
</p>




LinkedIn Carousel Generator is a robust web application designed to simplify the creation of LinkedIn carousels. It's built with the power and safety of Rust, leveraging the Actix web framework for server-side operations, and HTML/CSS for a sleek, user-friendly client-side interface.

## Key Features

- **Image Upload**: Effortlessly upload multiple images to be included in your carousel.
- **Text Overlay**: Customize your carousel with personalized text overlays on your images.
- **One-Click Generation**: Create a professional LinkedIn carousel with a single click.

## Architecture

The application runs on a server set up using the Actix framework, listening on `127.0.0.1:8080`. It features a single route (`/`) that responds with a welcome message.

The core logic for overlaying text on images is encapsulated in `src/lib.rs`. This includes the `TextProperties` struct for holding text properties, and two functions, `overlay_text_on_image` and `minimalist_text_overlay`, for performing the text overlay on an image.

The user interface is a clean, intuitive HTML form that allows users to upload multiple images and enter text for a carousel. The form data is sent to the server when the user clicks the "Generate Carousel" button. The aesthetic of the page is controlled by the CSS in `style.css`.

## Usage

1. Open your web browser and navigate to `127.0.0.1:8080`.
2. Use the form to upload your images and enter your desired text.
3. Click "Generate Carousel" to create your LinkedIn carousel.

Experience the simplicity and efficiency of creating LinkedIn carousels with our LinkedIn Carousel Generator.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
