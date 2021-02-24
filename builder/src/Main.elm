module Main exposing (main)

import Browser
import Html exposing (..)


main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = always Sub.none
        }



-- MODEL


type alias Model =
    { url : String
    , components : List String
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( Model "" [ "" ], Cmd.none )



-- UPDATE


type Msg
    = UrlChanged
    | AddComponent


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        _ ->
            ( model, Cmd.none )



-- VIEW


view : Model -> Html Msg
view model =
    text "Ahojda"
