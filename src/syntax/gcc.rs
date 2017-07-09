// Original file: "GCC.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::RList;
// use Language::C::System::Preprocess;
// use Data::Maybe;
// use System::Process;
// use System::Directory;
// use Data::List;

use syntax::preprocess::CppArgs;
use data::r_list::{Reversed, RList, snoc};
use syntax::preprocess::CppOption::*;
use syntax::preprocess::*;

pub struct GCC {
    gccPath: FilePath,
}
fn gccPath(a: GCC) -> FilePath {
    a.gccPath
}

pub fn newGCC(gccPath: FilePath) -> GCC {
    GCC { gccPath }
}

pub fn gccParseCPPArgs(args: Vec<String>) -> Result<(CppArgs, Vec<String>), String> {


    fn getDefine(opt: String) -> CppOption {
        let (key, val) = __break_str(|x| { '=' == x}, opt);

        Define(key, (if val.is_empty() { "".to_string() } else { tail_str(val) }))
    }

    fn getArgOpt (cpp_opt: String, mut rest: Vec<String>) -> Option<(CppOption, Vec<String>)> {
        unreachable!()
        // TODO
        // if isPrefixOf("-I".to_string(), cpp_opt) {
        //     Some((IncludeDir(drop_str(2, cpp_opt).into()), rest))
        // } else if isPrefixOf("-U".to_string(), cpp_opt) {
        //     Some((Undefine(drop_str(2, cpp_opt)), rest))
        // } else if isPrefixOf("-D".to_string(), cpp_opt) {
        //     Some((getDefine(drop_str(2, cpp_opt)), rest))
        // } else if cpp_opt == "-include" {
        //     let f = rest.remove(0);
        //     Some((IncludeFile(f.into()), rest))
        // } else {
        //     None
        // }
    }

    fn mungeArgs(parsed: ParseArgsState, unparsed_args: Vec<String>) -> Result<ParseArgsState, String> {
        let (cpp_args, unparsed) = parsed.clone();
        let (inp, out, cpp_opts) = cpp_args.clone();
        let (extra, other) = unparsed.clone();

        let a = unparsed_args;

        // ["-E", rest..]
        if a.len() > 0 && a[0] == "-E" {
            let rest = a[1..].to_vec();
            return mungeArgs(parsed, rest);
        }

        // [flag, flagArg, rest..]
        if a.len() > 1 {
            let flag = a[0].clone();
            let flagArg = a[1].clone();
            let rest = a[2..].to_vec();
            if (flag == "-MF".to_string()) ||
                (flag == "-MT".to_string()) ||
                (flag == "-MQ".to_string()) {
                return mungeArgs((cpp_args, (extra, snoc(snoc(other, flag), flagArg))), rest);
            }
        }

        // [flag, rest..]
        if a.len() > 0 {
            let flag = a[0].clone();
            let rest = a[1..].to_vec();
            if (flag == "-c".to_string()) ||
                (flag == "-S".to_string()) ||
                isPrefixOf("-M".to_string(), flag.clone()) {
                return mungeArgs((cpp_args, (extra, snoc(other, flag))), rest)
            }
        }

        // ["-o", file, rest..]
        if a.len() > 1 && a[0] == "-o" {
            let file = a[1].clone();
            let rest = a[2..].to_vec();
            return if isJust(out) {
                Err("two output files given".to_string())
            } else {
                mungeArgs(((inp, Some(file.into()), cpp_opts), unparsed), rest)
            };
        }

        // [cpp_opt, rest..]
        if a.len() > 0 {
            let cpp_opt = a[0].clone();
            let rest = a[1..].to_vec();
            if (getArgOpt(cpp_opt.clone(), rest.to_vec()).is_some()) {
                let (opt, rest_q) = getArgOpt(cpp_opt.clone(), rest.to_vec()).unwrap();
                return mungeArgs(((inp, out, snoc(cpp_opts, opt)), unparsed), rest_q);
            }
        }

        // [cfile, rest..]
        if a.len() > 0 {
            let cfile = a[0].clone();
            let rest = a[1..].to_vec();
            if (any(|x| { isSuffixOf(cfile.clone(), x.clone()) }, (words(".c .hc .h".to_string())))) {
                return if isJust(inp) {
                    Err("two input files given".to_string())
                } else {
                    mungeArgs(((Some(cfile.into()), out, cpp_opts), unparsed), rest)
                };
            }
        }

        // [unknown, rest..]
        if a.len() > 0 {
            let unknown = a[0].clone();
            let rest = a[1..].to_vec();
            return mungeArgs((cpp_args, (snoc(extra, unknown), other)), rest);
        }

        // otherwise
        Ok(parsed)
    }

    match mungeArgs(((None, None, RList::empty()), (RList::empty(), RList::empty())),
                    args) {
        Err(err) => Err(err),
        Ok(((None, _, _), _)) => Err("No .c / .hc / .h source file given".to_string()),
        Ok(((Some(input_file), output_file_opt, cpp_opts), (extra_args, other_args))) => {
            Ok((__assign!((rawCppArgs((RList::reverse(extra_args)), input_file)),
                          {
                              outputFile: output_file_opt,
                              cppOptions: RList::reverse(cpp_opts),
                          }),
                RList::reverse(other_args)))
        }
    }
}

pub type ParseArgsState = ((Option<FilePath>, Option<FilePath>, Reversed<Vec<CppOption>>),
                           (Reversed<Vec<String>>, Reversed<Vec<String>>));

pub fn buildCppArgs(CppArgs {
    cppOptions: options,
    extraOptions: extra_args,
    cppTmpDir: _tmpdir,
    inputFile: input_file,
    outputFile: output_file_opt
}: CppArgs)
                    -> Vec<String> {

    let tOption = |_0| match (_0) {
        IncludeDir(incl) => vec!["-I".to_string(), incl.to_string()],
        Define(key, value) => {
            vec![__op_addadd("-D".to_string(),
                             __op_addadd(key,
                                         (__op_addadd(if value.is_empty() {
                                                          "".to_string()
                                                      } else {
                                                          "=".to_string()
                                                      },
                                                      value))))]
        }
        Undefine(key) => vec![__op_addadd("-U".to_string(), key)],
        IncludeFile(f) => vec!["-include".to_string(), f.to_string()],
    };
    
    let outputFileOpt = output_file_opt.map(|x| vec!["-o".to_string(), x.to_string()]).unwrap_or(vec![]);

    __op_addadd((__concatMap!(tOption, options)),
                __op_addadd(outputFileOpt,
                            __op_addadd(vec!["-E".to_string(), input_file.to_string()],
                            extra_args)))
}
