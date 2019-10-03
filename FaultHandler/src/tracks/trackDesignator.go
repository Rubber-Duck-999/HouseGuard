package tracks

import (
	"fmt"
)

type Track struct {
	Internal_id uint
	Speed float64
	Rotation float64
	WGS84
}

type WGS84 struct {
	Latitude float64
	Longitude float64
	Altitude float64
}

func TrackDesignator() Track {
   var trackList map[uint]Track
   /* create a map*/
   trackList = make(map[uint]Track)
   
   /* insert key-value pairs in the map*/
   var id uint
   id = 1
   
   trackList[id] = (Track{id, 20.40, 39.5, WGS84{0.0, 0.0, 0.0},})
   id = 2
   trackList[id] = (Track{id, 20.40, 39.5, WGS84{0.0, 0.0, 0.0},})
   
   /* print map using keys*/
   for track := range trackList {
      fmt.Println("Track details id: ", trackList[track].Internal_id,", Speed: ", trackList[track].Speed)
   }
   return trackList[1]
}