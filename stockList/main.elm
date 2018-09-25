module Main exposing (..)

import Html exposing (..)
import Dict exposing (..)
import Json.Decode exposing (float)
import Http
import Html.Attributes exposing (attribute, value)
import Html.Events exposing (onClick, onInput)

import Debug exposing (log)


-- MODEL


type alias Stock =
    Maybe Float


type alias Model =
    { stocks : Dict String Stock
    , input : String
    }



-- UPDATE


type Msg
    = AddStock
    | NewPrice ( String, Float )
    | BadSymbol String
    | TextInput String


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        AddStock ->
            ( { model | stocks = Dict.insert model.input Nothing model.stocks, input = "" }, getStockPrice model.input )

        NewPrice ( sym, price ) ->
            ( { model | stocks = Dict.insert sym (Just price) model.stocks }, Cmd.none )

        BadSymbol sym ->
            ( { model | stocks = Dict.remove sym model.stocks }, Cmd.none )

        TextInput t ->
            ( { model | input = String.toUpper t }, Cmd.none )


getStockPrice sym =
    let
        handleResponse result =
            case result of
                Ok val ->
                    NewPrice ( sym, val )

                Err e ->
                    let 
                        _ = log "error:" e
                    in
                    BadSymbol sym

        url =
            "http://localhost:8080/load/" ++ sym
    in
        Http.send handleResponse (Http.get url priceDecoder)



-- VIEW


view : Model -> Html Msg
view model =
    let
        priceString price =
            case price of
                Just x ->
                    text (toString x)

                Nothing ->
                    div [ attribute "style" "background-image: url(\"https://media.giphy.com/media/eizrMRMxzlC6c/giphy.gif\")" ] []

        listifyStock ( sym, price ) =
            tr [] [ td [] [ text sym ], td [] [ priceString price ] ]
    in
        div []
            [ table [] (List.map listifyStock (Dict.toList model.stocks))
            , input [ onInput TextInput, value model.input ] []
            , button [ onClick AddStock ] [ text "Submit" ]
            ]


init =
    ( { stocks = Dict.empty, input = "" }, Cmd.none )


priceDecoder =
    float


main =
    Html.program
        { init = init
        , update = update
        , subscriptions = \_ -> Sub.none
        , view = view
        }
