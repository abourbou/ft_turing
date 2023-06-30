
(* https://fr.wikibooks.org/wiki/Objective_Caml/Bases#Exercices_3 *)

let appliquer_operation_postfixe x y o = o x y;;

let f x = x *. x -. 2. *. x +. 1.;;

let presque_zero epsilon = 
  fun x ->
    abs_float x < abs_float epsilon;;
  
let presque_racine x =
  fun epsilon ->
    abs_float (f x) < abs_float epsilon;;

let presque_racine_generique f epsilon x =
  abs_float (f x) < abs_float epsilon;;

let presque_racine x =
  presque_racine_generique 