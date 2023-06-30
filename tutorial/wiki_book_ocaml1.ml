
(* https://fr.wikibooks.org/wiki/Objective_Caml/Bases#Exercices  *)

let f x = x *. x -. 2. *. x +. 1.;;

let presque_zero epsilon =
  fun x ->
   abs_float x < abs_float epsilon;;

let presq = presque_zero 0.3;;

presq 1. ;;
presq 0.2;;
presq 0.;;
presq (-0.2);;

let presque_racine x =
  fun epsilon ->
    abs_float (f x) < abs_float epsilon;;

let pres_rac5 presque_racine 5;;
    (* 
    pres_rac5 10.;;
    pres_rac5 16.;; *)