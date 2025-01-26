module HomePage exposing (..)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Task
import Time
import Random


main =
  Browser.element
    { init = init
    , view = view
    , update = update
    , subscriptions = subscriptions
    }



-- MODEL


type WhoWaits
 = Nobody 
 | Paint
 | Check

type alias Color = Int

type alias Model =
  { paint_progress : Int
  , n_checked : Int
  , birdhouses : List Color
  , current_color : Color
  , waiting : WhoWaits
  }

max_paint_progress = 100


init : () -> (Model, Cmd Msg)
init _ =
  ( Model 0 0 [] 0 Nobody
  , Cmd.none
  )



-- UPDATE


type Msg
  = TickPaint
  | TickCheck
  | NewBirdhouse Color


add_birdhouse model new_color = { model | waiting = Nobody, paint_progress = 0, n_checked = 0, birdhouses = model.birdhouses ++ [model.current_color], current_color = new_color }

update : Msg -> Model -> (Model, Cmd Msg)
update msg model = case msg of 
    NewBirdhouse new_color -> (add_birdhouse model new_color, Cmd.none)
    TickPaint -> 
      if model.paint_progress < max_paint_progress then ( { model | paint_progress = model.paint_progress + 1}, Cmd.none)
      else case model.waiting of
            Nobody -> ({ model | waiting = Paint }, Cmd.none)
            Paint -> (model, Cmd.none)
            Check -> (model, Random.generate NewBirdhouse (Random.int 0 360))
    TickCheck -> if model.n_checked < (List.length model.birdhouses) then ({ model | n_checked = model.n_checked + 1}, Cmd.none)
                 else case model.waiting of
                    Nobody -> ({ model | waiting = Check }, Cmd.none)
                    Check -> (model, Cmd.none)
                    Paint -> (model, Random.generate NewBirdhouse (Random.int 0 360))

-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch [
  Time.every 10 (always TickPaint),
  Time.every 150 (always TickCheck)
  ]




-- VIEW

little_bird_house n_checked idx color = 
    let bg_color = if n_checked == idx then "yellow" else "transparent" 
        in div [style "display" "inline-block", style "height" "fit-content", style "background-color" bg_color] [img [src "../res/coloredBirdhouse.png", style "height" "5em", style "filter" ("hue-rotate(" ++ (String.fromInt color) ++ "deg)")] []]


view : Model -> Html Msg
view model =
  div [] [ 
      div [style "display" "grid", style "grid-template-rows" "auto", style "grid-template-columns" "30% auto"] [
      img [src "../res/whiteBirdhouse.png", style "grid-column" "1/1", style "grid-row" "1/1"] [],
      img [src "../res/coloredBirdhouse.png", style "grid-column" "1/1", style "grid-row" "1/1", style "clip-path" ("xywh(0 " ++ (String.fromInt (max_paint_progress - model.paint_progress)) ++ "% 100% 100%)"), style "filter" ("hue-rotate(" ++ (String.fromInt model.current_color) ++ "deg)")] [],
      div [style "grid-area" "1/2/1/2", style "display" "flex", style "flex-wrap" "wrap"] (List.indexedMap (little_bird_house model.n_checked) model.birdhouses)
      ]
  ]

