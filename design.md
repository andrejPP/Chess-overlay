# Chess overlay design document

This document provides design overview of the algorithm and aplication available in this repository.

## Design 0.0.1

Application enables user to analyze running chess game in real time and creates overlay helping user to learn. This is done as follows:

### 1. User selects monitor for acquisition

- To start the analysis user should select a display that contains or will contain a chess board.

- (Optional) User may select exact position of the chess board if it is not full screen

### 2. Display image acquisition 

- After user starts the analysis by button (TBD), images of selected display are acquired repeadetly with certain delay (TBD).

### 3. Chess board state detection

- Image is passed to image detector that:
    - Finds chess board boundaries in the image (may be skipped if optional step from 1. point is implemented)
    - Analyzes detected chess board and creates corresponding [PGN](https://en.wikipedia.org/wiki/Portable_Game_Notation) text

### 4. Create overlay

- Based on the state acquired in the previous step prepare overlay for the user (TBD).