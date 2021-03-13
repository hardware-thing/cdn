module Main exposing (main)

import Browser
import Dict exposing (Dict)
import Html exposing (..)
import Html.Attributes as Attr
import Html.Events as Event
import Http
import List.Extra as List


main : Program String Model Msg
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
    , components : Component
    }


type Component
    = Component Bool (Dict String Component)


init : String -> ( Model, Cmd Msg )
init url =
    ( { error = Nothing
      , url = url
      , components = Component False Dict.empty
      }
    , fetchList url
    )



-- UPDATE


type Msg
    = FetchList
    | GotList (Result Http.Error String)
    | UrlChanged
    | ToggleComponent (List String) Bool


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        FetchList ->
            ( model
            , fetchList model.url
            )

        GotList result ->
            case result of
                Ok list ->
                    ( { model | components = deserializeComponents list }
                    , Cmd.none
                    )

                Err error ->
                    ( { model
                        | error =
                            Just "I could not fetch the list of supported components. Sorry."
                      }
                    , Cmd.none
                    )

        ToggleComponent at checked ->
            ( { model | components = deepToggle at checked model.components }
            , Cmd.none
            )

        _ ->
            ( model, Cmd.none )


fetchList : String -> Cmd Msg
fetchList url =
    Http.get
        { url = url ++ "/v1/list"
        , expect = Http.expectString GotList
        }


deserializeComponents : String -> Component
deserializeComponents list =
    list
        |> String.lines
        |> List.foldl
            (String.split ":" >> deepInsert)
            (Component False Dict.empty)


deepInsert : List String -> Component -> Component
deepInsert fragments (Component _ dict) =
    case fragments of
        [ subs ] ->
            subs
                |> String.split "|"
                |> List.foldl
                    (\sub dictLevel ->
                        Dict.insert sub (Component False Dict.empty) dictLevel
                    )
                    dict
                |> Component False

        key :: path ->
            Dict.update key
                (\maybeValue ->
                    case maybeValue of
                        -- If the key is already there
                        Just component ->
                            deepInsert path component
                                |> Just

                        -- If the component is not there
                        Nothing ->
                            deepInsert path (Component False Dict.empty)
                                |> Just
                )
                dict
                |> Component False

        _ ->
            Component False dict


deepToggle : List String -> Bool -> Component -> Component
deepToggle at checked (Component current dict) =
    case at of
        [ key ] ->
            Component checked dict

        key :: path ->
            Dict.update key
                (Maybe.map (deepToggle path checked))
                dict
                |> Component current

        _ ->
            Component current dict



-- VIEW


view : Model -> Html Msg
view model =
    main_ []
        [ button
            [ Event.onClick FetchList ]
            [ text "Refetch components" ]
        ]
