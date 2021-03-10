module Main exposing (main)

import Browser
import Html exposing (..)
import Http


main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = always Sub.none
        }



-- MODEL


type alias Model =
    { error : Maybe String

    --
    , url : String
    , components : List String
    }


init : String -> ( Model, Cmd Msg )
init url =
    ( Model Nothing url [ "" ], Cmd.none )



-- UPDATE


type Msg
    = FetchList
    | GotList (Result Http.Error String)
    | UrlChanged
    | AddComponent


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        FetchList ->
            ( model
            , Http.get
                { url = model.url ++ "/v1/list"
                , expect = Http.expectString GotList
                }
            )

        GotList result ->
            case result of
                Ok list ->
                    ( { model | components = String.split "\n" list }, Cmd.none )

                Err error ->
                    ( { model
                        | error =
                            Just "I could not fetch the list of supported components. Sorry."
                      }
                    , Cmd.none
                    )

        _ ->
            ( model, Cmd.none )



-- VIEW


view : Model -> Html Msg
view model =
    text model.url
