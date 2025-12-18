package main

import (

  //this provides Go bindings for JavaScript and allows Go code to interact with the browser
    "syscall/js" 
)

func main() {
    //this creates an empty channel to prevent the program from exiting
    c := make(chan struct{}, 0)

    /*this then exposes the Go function toGray to JavaScript as convertToGray
    JS can now call convertToGray(imageData.data, width, height) */
    js.Global().Set("convertToGray", js.FuncOf(toGray))

    //this will block it forever, keeping the Go program alive so JS can call the function
    <-c
}

//then toGray converts an image to grayscale.
/*Arguments from JavaScript:
1. data: containing RGBA pixel values
2. width: width of the image
3. height: height of the image */

func toGray(this js.Value, args []js.Value) interface{} {
    data := args[0]       //representing the image pixels
    width := args[1].Int()  //the image width
    height := args[2].Int() //the image height

    //then loop through each pixel row
    for y := 0; y < height; y++ {
        //loop through each pixel column
        for x := 0; x < width; x++ {
            /* then calculate the index of the pixel in the array
            each pixel has 4 values: R, G, B, A */
            idx := (y*width + x) * 4

            //we then extract the red, green, and blue components of the pixel
            r := data.Index(idx).Int()
            g := data.Index(idx + 1).Int()
            b := data.Index(idx + 2).Int()

            //then we compute the gray value as the average of R, G, and B
            gray := (r + g + b) / 3

            //then we set the R, G, and B channels to the gray value
            data.SetIndex(idx, gray)
            data.SetIndex(idx+1, gray)
            data.SetIndex(idx+2, gray)

            //we then leave the (A) channel unchanged
        }
    }

    return nil //we don't need a return value because the function must return something for js.FuncOf, but the image data is modified in place, so there’s no actual value to return.
}
