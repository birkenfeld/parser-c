// cargo-deps: command-macros="0.1.10" error-chain="*"

//! Regenerates source Lexer.hs and Parser.hs files from their .x and .y sources.
//!
//! ```
//! stack install alex hindent happy
//! ```

#[macro_use] extern crate command_macros;
#[macro_use] extern crate error_chain;

use std::fs::File;
use std::io::prelude::*;

// Error chain
mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
    }
}
use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    println!("Lexer...");
    cmd!( alex ("../deps/language-c/src/Language/C/Parser/Lexer.x") ("-o") ("./gen/Lexer.hs") ).status();
    println!("...hindent..."); 
    cmd!( hindent ("./gen/Lexer.hs") ).status();

    let mut contents = String::new();
    {
        File::open("./gen/Lexer.hs")?.read_to_string(&mut contents)?;
    }

    let contents = contents.replacen("where", &format!("where\n\n{}", lexer_heading), 1);
    {
        File::create("./gen/Lexer.hs")?.write_all(contents.as_bytes())?;
    }

    println!("Parser...");
    cmd!( happy ("../deps/language-c/src/Language/C/Parser/Parser.y") ("-o") ("./gen/Parser.hs") ).status();
    println!("...hindent...");
    cmd!( hindent ("./gen/Parser.hs") ).status();

    let mut contents = String::new();
    {
        File::open("./gen/Parser.hs")?.read_to_string(&mut contents)?;
    }

    let contents = contents.replacen("where", &format!("where\n\n{}", parser_heading), 1);
    {
        File::create("./gen/Parser.hs")?.write_all(contents.as_bytes())?;
    }

    Ok(())
}

const lexer_heading: &'static str = "
alexAndPred :: (a -> AlexInput -> Int -> AlexInput -> Bool)
  -> (a -> AlexInput -> Int -> AlexInput -> Bool)
  -> a
  -> AlexInput
  -> Int
  -> AlexInput
  -> Bool
alexIndexInt16OffAddr :: Array Int Int -> Int -> Int
alexIndexInt32OffAddr :: Array Int Int -> Int -> Int
quickIndex :: Array Int b -> Int -> b
idkwtok :: [Char] -> Position -> P CToken
alexScan :: (Position, InputStream)
  -> Int
  -> AlexReturn (Position -> Int -> InputStream -> P CToken)
alexScanUser :: Bool
  -> AlexInput
  -> Int
  -> AlexReturn (Position -> Int -> InputStream -> P CToken)
alex_scan_tkn :: Bool
  -> AlexInput
  -> Int
  -> AlexInput
  -> Int
  -> AlexLastAcc
  -> (AlexLastAcc, AlexInput)
alexPrevCharIs :: Char -> Int -> AlexInput -> Int -> Int -> Bool
alexPrevCharMatches :: (Char -> Int) -> Int -> AlexInput -> Int -> Int -> Int
alexPrevCharIsOneOf :: Array Char Bool -> Int -> AlexInput -> Int -> AlexInput -> Bool
alexRightContext :: Int -> Bool -> AlexInput -> Int -> AlexInput -> Bool
alex_accept :: Array Int (AlexAcc a)
alex_actions :: Array Int (Position -> Int -> InputStream -> P CToken)
alex_action_1 :: Position -> Int -> InputStream -> P CToken
alex_action_4 :: Position -> Int -> InputStream -> P CToken
alex_action_5 :: Position -> Int -> InputStream -> P CToken
alex_action_6 :: Position -> Int -> InputStream -> P CToken
alex_action_7 :: Position -> Int -> InputStream -> P CToken
alex_action_8 :: Position -> Int -> InputStream -> P CToken
alex_action_9 :: Position -> Int -> InputStream -> P CToken
alex_action_10 :: Position -> Int -> InputStream -> P CToken
alex_action_11 :: Position -> Int -> InputStream -> P CToken
alex_action_12 :: Position -> Int -> InputStream -> P CToken
alex_action_13 :: Position -> Int -> InputStream -> P CToken
alex_action_14 :: Position -> Int -> InputStream -> P CToken
alex_action_15 :: Position -> Int -> InputStream -> P CToken
alex_action_16 :: Position -> Int -> InputStream -> P CToken
alex_action_17 :: Position -> Int -> InputStream -> P CToken
alex_action_18 :: Position -> Int -> InputStream -> P CToken
alex_action_19 :: Position -> Int -> InputStream -> P CToken
alex_action_20 :: Position -> Int -> InputStream -> P CToken
alex_action_21 :: Position -> Int -> InputStream -> P CToken
alex_action_22 :: Position -> Int -> InputStream -> P CToken
alex_action_23 :: Position -> Int -> InputStream -> P CToken
alex_action_24 :: Position -> Int -> InputStream -> P CToken
alex_action_25 :: Position -> Int -> InputStream -> P CToken
alex_action_26 :: Position -> Int -> InputStream -> P CToken
alex_action_27 :: Position -> Int -> InputStream -> P CToken
alex_action_28 :: Position -> Int -> InputStream -> P CToken
alex_action_29 :: Position -> Int -> InputStream -> P CToken
alex_action_30 :: Position -> Int -> InputStream -> P CToken
alex_action_31 :: Position -> Int -> InputStream -> P CToken
alex_action_32 :: Position -> Int -> InputStream -> P CToken
alex_action_33 :: Position -> Int -> InputStream -> P CToken
alex_action_34 :: Position -> Int -> InputStream -> P CToken
alex_action_35 :: Position -> Int -> InputStream -> P CToken
alex_action_36 :: Position -> Int -> InputStream -> P CToken
alex_action_37 :: Position -> Int -> InputStream -> P CToken
alex_action_38 :: Position -> Int -> InputStream -> P CToken
alex_action_39 :: Position -> Int -> InputStream -> P CToken
alex_action_40 :: Position -> Int -> InputStream -> P CToken
alex_action_41 :: Position -> Int -> InputStream -> P CToken
alex_action_42 :: Position -> Int -> InputStream -> P CToken
alex_action_43 :: Position -> Int -> InputStream -> P CToken
alex_action_44 :: Position -> Int -> InputStream -> P CToken
alex_action_45 :: Position -> Int -> InputStream -> P CToken
alex_action_46 :: Position -> Int -> InputStream -> P CToken
alex_action_47 :: Position -> Int -> InputStream -> P CToken
alex_action_48 :: Position -> Int -> InputStream -> P CToken
alex_action_49 :: Position -> Int -> InputStream -> P CToken
alex_action_50 :: Position -> Int -> InputStream -> P CToken
alex_action_51 :: Position -> Int -> InputStream -> P CToken
alex_action_52 :: Position -> Int -> InputStream -> P CToken
alex_action_53 :: Position -> Int -> InputStream -> P CToken
alex_action_54 :: Position -> Int -> InputStream -> P CToken
alex_action_55 :: Position -> Int -> InputStream -> P CToken
alex_action_56 :: Position -> Int -> InputStream -> P CToken
alex_action_57 :: Position -> Int -> InputStream -> P CToken
alex_action_58 :: Position -> Int -> InputStream -> P CToken
alex_action_59 :: Position -> Int -> InputStream -> P CToken
alex_action_60 :: Position -> Int -> InputStream -> P CToken
alex_action_61 :: Position -> Int -> InputStream -> P CToken
alex_action_62 :: Position -> Int -> InputStream -> P CToken
alex_action_63 :: Position -> Int -> InputStream -> P CToken
alex_action_64 :: Position -> Int -> InputStream -> P CToken
alex_action_65 :: Position -> Int -> InputStream -> P CToken
alex_action_66 :: Position -> Int -> InputStream -> P CToken
alex_action_67 :: Position -> Int -> InputStream -> P CToken
";

const parser_heading: &'static str = "
happyNewToken :: Bool
happyError_ :: Bool
translation_unit :: Bool
external_declaration :: Bool
statement :: Bool
expression :: Bool
happySeq :: Bool
happyParse :: Bool
happyAccept :: Bool
happyShift :: Bool
happyReduce :: Bool
happyMonadReduce :: Bool
happyGoto :: Bool
happyFail :: Bool
happyDrop :: Bool
happyDropStk :: Bool
happyMonad2Reduce :: Bool
happyReduction_0 :: Bool
happyReduction_1 :: Bool
happyReduction_2 :: Bool
happyReduction_3 :: Bool
happyReduction_4 :: Bool
happyReduction_5 :: Bool
happyReduction_6 :: Bool
happyReduction_7 :: Bool
happyReduction_8 :: Bool
happyReduction_9 :: Bool
happyReduction_10 :: Bool
happyReduction_11 :: Bool
happyReduction_12 :: Bool
happyReduction_13 :: Bool
happyReduction_14 :: Bool
happyReduction_15 :: Bool
happyReduction_16 :: Bool
happyReduction_17 :: Bool
happyReduction_18 :: Bool
happyReduction_19 :: Bool
happyReduction_20 :: Bool
happyReduction_21 :: Bool
happyReduction_22 :: Bool
happyReduction_23 :: Bool
happyReduction_24 :: Bool
happyReduction_25 :: Bool
happyReduction_26 :: Bool
happyReduction_27 :: Bool
happyReduction_28 :: Bool
happyReduction_29 :: Bool
happyReduction_30 :: Bool
happyReduction_31 :: Bool
happyReduction_32 :: Bool
happyReduction_33 :: Bool
happyReduction_34 :: Bool
happyReduction_35 :: Bool
happyReduction_36 :: Bool
happyReduction_37 :: Bool
happyReduction_38 :: Bool
happyReduction_39 :: Bool
happyReduction_40 :: Bool
happyReduction_41 :: Bool
happyReduction_42 :: Bool
happyReduction_43 :: Bool
happyReduction_44 :: Bool
happyReduction_45 :: Bool
happyReduction_46 :: Bool
happyReduction_47 :: Bool
happyReduction_48 :: Bool
happyReduction_49 :: Bool
happyReduction_50 :: Bool
happyReduction_51 :: Bool
happyReduction_52 :: Bool
happyReduction_53 :: Bool
happyReduction_54 :: Bool
happyReduction_55 :: Bool
happyReduction_56 :: Bool
happyReduction_57 :: Bool
happyReduction_58 :: Bool
happyReduction_59 :: Bool
happyReduction_60 :: Bool
happyReduction_61 :: Bool
happyReduction_62 :: Bool
happyReduction_63 :: Bool
happyReduction_64 :: Bool
happyReduction_65 :: Bool
happyReduction_66 :: Bool
happyReduction_67 :: Bool
happyReduction_68 :: Bool
happyReduction_69 :: Bool
happyReduction_70 :: Bool
happyReduction_71 :: Bool
happyReduction_72 :: Bool
happyReduction_73 :: Bool
happyReduction_74 :: Bool
happyReduction_75 :: Bool
happyReduction_76 :: Bool
happyReduction_77 :: Bool
happyReduction_78 :: Bool
happyReduction_79 :: Bool
happyReduction_80 :: Bool
happyReduction_81 :: Bool
happyReduction_82 :: Bool
happyReduction_83 :: Bool
happyReduction_84 :: Bool
happyReduction_85 :: Bool
happyReduction_86 :: Bool
happyReduction_87 :: Bool
happyReduction_88 :: Bool
happyReduction_89 :: Bool
happyReduction_90 :: Bool
happyReduction_91 :: Bool
happyReduction_92 :: Bool
happyReduction_93 :: Bool
happyReduction_94 :: Bool
happyReduction_95 :: Bool
happyReduction_96 :: Bool
happyReduction_97 :: Bool
happyReduction_98 :: Bool
happyReduction_99 :: Bool
happyReduction_100 :: Bool
happyReduction_101 :: Bool
happyReduction_102 :: Bool
happyReduction_103 :: Bool
happyReduction_104 :: Bool
happyReduction_105 :: Bool
happyReduction_106 :: Bool
happyReduction_107 :: Bool
happyReduction_108 :: Bool
happyReduction_109 :: Bool
happyReduction_110 :: Bool
happyReduction_111 :: Bool
happyReduction_112 :: Bool
happyReduction_113 :: Bool
happyReduction_114 :: Bool
happyReduction_115 :: Bool
happyReduction_116 :: Bool
happyReduction_117 :: Bool
happyReduction_118 :: Bool
happyReduction_119 :: Bool
happyReduction_120 :: Bool
happyReduction_121 :: Bool
happyReduction_122 :: Bool
happyReduction_123 :: Bool
happyReduction_124 :: Bool
happyReduction_125 :: Bool
happyReduction_126 :: Bool
happyReduction_127 :: Bool
happyReduction_128 :: Bool
happyReduction_129 :: Bool
happyReduction_130 :: Bool
happyReduction_131 :: Bool
happyReduction_132 :: Bool
happyReduction_133 :: Bool
happyReduction_134 :: Bool
happyReduction_135 :: Bool
happyReduction_136 :: Bool
happyReduction_137 :: Bool
happyReduction_138 :: Bool
happyReduction_139 :: Bool
happyReduction_140 :: Bool
happyReduction_141 :: Bool
happyReduction_142 :: Bool
happyReduction_143 :: Bool
happyReduction_144 :: Bool
happyReduction_145 :: Bool
happyReduction_146 :: Bool
happyReduction_147 :: Bool
happyReduction_148 :: Bool
happyReduction_149 :: Bool
happyReduction_150 :: Bool
happyReduction_151 :: Bool
happyReduction_152 :: Bool
happyReduction_153 :: Bool
happyReduction_154 :: Bool
happyReduction_155 :: Bool
happyReduction_156 :: Bool
happyReduction_157 :: Bool
happyReduction_158 :: Bool
happyReduction_159 :: Bool
happyReduction_160 :: Bool
happyReduction_161 :: Bool
happyReduction_162 :: Bool
happyReduction_163 :: Bool
happyReduction_164 :: Bool
happyReduction_165 :: Bool
happyReduction_166 :: Bool
happyReduction_167 :: Bool
happyReduction_168 :: Bool
happyReduction_169 :: Bool
happyReduction_170 :: Bool
happyReduction_171 :: Bool
happyReduction_172 :: Bool
happyReduction_173 :: Bool
happyReduction_174 :: Bool
happyReduction_175 :: Bool
happyReduction_176 :: Bool
happyReduction_177 :: Bool
happyReduction_178 :: Bool
happyReduction_179 :: Bool
happyReduction_180 :: Bool
happyReduction_181 :: Bool
happyReduction_182 :: Bool
happyReduction_183 :: Bool
happyReduction_184 :: Bool
happyReduction_185 :: Bool
happyReduction_186 :: Bool
happyReduction_187 :: Bool
happyReduction_188 :: Bool
happyReduction_189 :: Bool
happyReduction_190 :: Bool
happyReduction_191 :: Bool
happyReduction_192 :: Bool
happyReduction_193 :: Bool
happyReduction_194 :: Bool
happyReduction_195 :: Bool
happyReduction_196 :: Bool
happyReduction_197 :: Bool
happyReduction_198 :: Bool
happyReduction_199 :: Bool
happyReduction_200 :: Bool
happyReduction_201 :: Bool
happyReduction_202 :: Bool
happyReduction_203 :: Bool
happyReduction_204 :: Bool
happyReduction_205 :: Bool
happyReduction_206 :: Bool
happyReduction_207 :: Bool
happyReduction_208 :: Bool
happyReduction_209 :: Bool
happyReduction_210 :: Bool
happyReduction_211 :: Bool
happyReduction_212 :: Bool
happyReduction_213 :: Bool
happyReduction_214 :: Bool
happyReduction_215 :: Bool
happyReduction_216 :: Bool
happyReduction_217 :: Bool
happyReduction_218 :: Bool
happyReduction_219 :: Bool
happyReduction_220 :: Bool
happyReduction_221 :: Bool
happyReduction_222 :: Bool
happyReduction_223 :: Bool
happyReduction_224 :: Bool
happyReduction_225 :: Bool
happyReduction_226 :: Bool
happyReduction_227 :: Bool
happyReduction_228 :: Bool
happyReduction_229 :: Bool
happyReduction_230 :: Bool
happyReduction_231 :: Bool
happyReduction_232 :: Bool
happyReduction_233 :: Bool
happyReduction_234 :: Bool
happyReduction_235 :: Bool
happyReduction_236 :: Bool
happyReduction_237 :: Bool
happyReduction_238 :: Bool
happyReduction_239 :: Bool
happyReduction_240 :: Bool
happyReduction_241 :: Bool
happyReduction_242 :: Bool
happyReduction_243 :: Bool
happyReduction_244 :: Bool
happyReduction_245 :: Bool
happyReduction_246 :: Bool
happyReduction_247 :: Bool
happyReduction_248 :: Bool
happyReduction_249 :: Bool
happyReduction_250 :: Bool
happyReduction_251 :: Bool
happyReduction_252 :: Bool
happyReduction_253 :: Bool
happyReduction_254 :: Bool
happyReduction_255 :: Bool
happyReduction_256 :: Bool
happyReduction_257 :: Bool
happyReduction_258 :: Bool
happyReduction_259 :: Bool
happyReduction_260 :: Bool
happyReduction_261 :: Bool
happyReduction_262 :: Bool
happyReduction_263 :: Bool
happyReduction_264 :: Bool
happyReduction_265 :: Bool
happyReduction_266 :: Bool
happyReduction_267 :: Bool
happyReduction_268 :: Bool
happyReduction_269 :: Bool
happyReduction_270 :: Bool
happyReduction_271 :: Bool
happyReduction_272 :: Bool
happyReduction_273 :: Bool
happyReduction_274 :: Bool
happyReduction_275 :: Bool
happyReduction_276 :: Bool
happyReduction_277 :: Bool
happyReduction_278 :: Bool
happyReduction_279 :: Bool
happyReduction_280 :: Bool
happyReduction_281 :: Bool
happyReduction_282 :: Bool
happyReduction_283 :: Bool
happyReduction_284 :: Bool
happyReduction_285 :: Bool
happyReduction_286 :: Bool
happyReduction_287 :: Bool
happyReduction_288 :: Bool
happyReduction_289 :: Bool
happyReduction_290 :: Bool
happyReduction_291 :: Bool
happyReduction_292 :: Bool
happyReduction_293 :: Bool
happyReduction_294 :: Bool
happyReduction_295 :: Bool
happyReduction_296 :: Bool
happyReduction_297 :: Bool
happyReduction_298 :: Bool
happyReduction_299 :: Bool
happyReduction_300 :: Bool
happyReduction_301 :: Bool
happyReduction_302 :: Bool
happyReduction_303 :: Bool
happyReduction_304 :: Bool
happyReduction_305 :: Bool
happyReduction_306 :: Bool
happyReduction_307 :: Bool
happyReduction_308 :: Bool
happyReduction_309 :: Bool
happyReduction_310 :: Bool
happyReduction_311 :: Bool
happyReduction_312 :: Bool
happyReduction_313 :: Bool
happyReduction_314 :: Bool
happyReduction_315 :: Bool
happyReduction_316 :: Bool
happyReduction_317 :: Bool
happyReduction_318 :: Bool
happyReduction_319 :: Bool
happyReduction_320 :: Bool
happyReduction_321 :: Bool
happyReduction_322 :: Bool
happyReduction_323 :: Bool
happyReduction_324 :: Bool
happyReduction_325 :: Bool
happyReduction_326 :: Bool
happyReduction_327 :: Bool
happyReduction_328 :: Bool
happyReduction_329 :: Bool
happyReduction_330 :: Bool
happyReduction_331 :: Bool
happyReduction_332 :: Bool
happyReduction_333 :: Bool
happyReduction_334 :: Bool
happyReduction_335 :: Bool
happyReduction_336 :: Bool
happyReduction_337 :: Bool
happyReduction_338 :: Bool
happyReduction_339 :: Bool
happyReduction_340 :: Bool
happyReduction_341 :: Bool
happyReduction_342 :: Bool
happyReduction_343 :: Bool
happyReduction_344 :: Bool
happyReduction_345 :: Bool
happyReduction_346 :: Bool
happyReduction_347 :: Bool
happyReduction_348 :: Bool
happyReduction_349 :: Bool
happyReduction_350 :: Bool
happyReduction_351 :: Bool
happyReduction_352 :: Bool
happyReduction_353 :: Bool
happyReduction_354 :: Bool
happyReduction_355 :: Bool
happyReduction_356 :: Bool
happyReduction_357 :: Bool
happyReduction_358 :: Bool
happyReduction_359 :: Bool
happyReduction_360 :: Bool
happyReduction_361 :: Bool
happyReduction_362 :: Bool
happyReduction_363 :: Bool
happyReduction_364 :: Bool
happyReduction_365 :: Bool
happyReduction_366 :: Bool
happyReduction_367 :: Bool
happyReduction_368 :: Bool
happyReduction_369 :: Bool
happyReduction_370 :: Bool
happyReduction_371 :: Bool
happyReduction_372 :: Bool
happyReduction_373 :: Bool
happyReduction_374 :: Bool
happyReduction_375 :: Bool
happyReduction_376 :: Bool
happyReduction_377 :: Bool
happyReduction_378 :: Bool
happyReduction_379 :: Bool
happyReduction_380 :: Bool
happyReduction_381 :: Bool
happyReduction_382 :: Bool
happyReduction_383 :: Bool
happyReduction_384 :: Bool
happyReduction_385 :: Bool
happyReduction_386 :: Bool
happyReduction_387 :: Bool
happyReduction_388 :: Bool
happyReduction_389 :: Bool
happyReduction_390 :: Bool
happyReduction_391 :: Bool
happyReduction_392 :: Bool
happyReduction_393 :: Bool
happyReduction_394 :: Bool
happyReduction_395 :: Bool
happyReduction_396 :: Bool
happyReduction_397 :: Bool
happyReduction_398 :: Bool
happyReduction_399 :: Bool
happyReduction_400 :: Bool
happyReduction_401 :: Bool
happyReduction_402 :: Bool
happyReduction_403 :: Bool
happyReduction_404 :: Bool
happyReduction_405 :: Bool
happyReduction_406 :: Bool
happyReduction_407 :: Bool
happyReduction_408 :: Bool
happyReduction_409 :: Bool
happyReduction_410 :: Bool
happyReduction_411 :: Bool
happyReduction_412 :: Bool
happyReduction_413 :: Bool
happyReduction_414 :: Bool
happyReduction_415 :: Bool
happyReduction_416 :: Bool
happyReduction_417 :: Bool
happyReduction_418 :: Bool
happyReduction_419 :: Bool
happyReduction_420 :: Bool
happyReduction_421 :: Bool
happyReduction_422 :: Bool
happyReduction_423 :: Bool
happyReduction_424 :: Bool
happyReduction_425 :: Bool
happyReduction_426 :: Bool
happyReduction_427 :: Bool
happyReduction_428 :: Bool
happyReduction_429 :: Bool
happyReduction_430 :: Bool
happyReduction_431 :: Bool
happyReduction_432 :: Bool
happyReduction_433 :: Bool
happyReduction_434 :: Bool
happyReduction_435 :: Bool
happyReduction_436 :: Bool
happyReduction_437 :: Bool
happyReduction_438 :: Bool
happyReduction_439 :: Bool
happyReduction_440 :: Bool
happyReduction_441 :: Bool
happyReduction_442 :: Bool
happyReduction_443 :: Bool
happyReduction_444 :: Bool
happyReduction_445 :: Bool
happyReduction_446 :: Bool
happyReduction_447 :: Bool
happyReduction_448 :: Bool
happyReduction_449 :: Bool
happyReduction_450 :: Bool
happyReduction_451 :: Bool
happyReduction_452 :: Bool
happyReduction_453 :: Bool
happyReduction_454 :: Bool
happyReduction_455 :: Bool
happyReduction_456 :: Bool
happyReduction_457 :: Bool
happyReduction_458 :: Bool
happyReduction_459 :: Bool
happyReduction_460 :: Bool
happyReduction_461 :: Bool
happyReduction_462 :: Bool
happyReduction_463 :: Bool
happyReduction_464 :: Bool
happyReduction_465 :: Bool
happyReduction_466 :: Bool
happyReduction_467 :: Bool
happyReduction_468 :: Bool
happyReduction_469 :: Bool
happyReduction_470 :: Bool
happyReduction_471 :: Bool
happyReduction_472 :: Bool
happyReduction_473 :: Bool
happyReduction_474 :: Bool
happyReduction_475 :: Bool
happyReduction_476 :: Bool
happyReduction_477 :: Bool
happyReduction_478 :: Bool
happyReduction_479 :: Bool
happyReduction_480 :: Bool
happyReduction_481 :: Bool
happyReduction_482 :: Bool
happyReduction_483 :: Bool
happyReduction_484 :: Bool
happyReduction_485 :: Bool
happyReduction_486 :: Bool
happyReduction_487 :: Bool
happyReduction_488 :: Bool
happyReduction_489 :: Bool
happyReduction_490 :: Bool
happyReduction_491 :: Bool
happyReduction_492 :: Bool
happyReduction_493 :: Bool
happyReduction_494 :: Bool
happyReduction_495 :: Bool
happyReduction_496 :: Bool
happyReduction_497 :: Bool
happyReduction_498 :: Bool
happyReduction_499 :: Bool
happySpecReduce_0 :: Bool
happySpecReduce_1 :: Bool
happySpecReduce_2 :: Bool
happySpecReduce_3 :: Bool
happyThen1 :: Bool
";