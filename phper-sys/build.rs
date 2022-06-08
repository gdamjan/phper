// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use bindgen::Builder;
use std::{env, ffi::OsStr, fmt::Debug, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=php_wrapper.c");
    println!("cargo:rerun-if-env-changed=PHP_CONFIG");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let php_config = env::var("PHP_CONFIG").unwrap_or_else(|_| "php-config".to_string());

    let includes = execute_command(&[php_config.as_str(), "--includes"]);
    let includes = includes.split(' ').collect::<Vec<_>>();

    // Generate php const.

    let php_bin = execute_command(&[php_config.as_str(), "--php-binary"]);
    let php_info = execute_command(&[php_bin.as_str(), "-i"]);

    println!(
        "cargo:rustc-env=ZEND_MODULE_BUILD_ID={}",
        php_info
            .lines()
            .find_map(|line| {
                if line.starts_with("Zend Extension Build") {
                    Some(
                        line.chars()
                            .skip_while(|c| *c != 'A')
                            .collect::<String>()
                            .trim()
                            .to_owned(),
                    )
                } else {
                    None
                }
            })
            .expect("Can't found the field `Zend Extension Build`")
    );

    println!(
        "cargo:rustc-env=PHP_MODULE_BUILD_ID={}",
        php_info
            .lines()
            .find_map(|line| {
                if line.starts_with("PHP Extension Build") {
                    Some(
                        line.chars()
                            .skip_while(|c| *c != 'A')
                            .collect::<String>()
                            .trim()
                            .to_owned(),
                    )
                } else {
                    None
                }
            })
            .expect("Can't found the field `PHP Extension Build`")
    );

    // Generate libphpwrapper.a.

    let mut builder = cc::Build::new();
    for include in &includes {
        builder.flag(include);
    }
    builder.file("php_wrapper.c").compile("phpwrapper");

    // Generate bindgen file.

    includes.iter().for_each(|include| {
        let include = &include[2..];
        println!("cargo:include={}", include);
    });

    let bindings = Builder::default()
        .header("php_wrapper.c")
        .clang_args(&includes)
        .blocklist_function("__acosf64x")
        .blocklist_function("__acosf64x")
        .blocklist_function("__acoshf64x")
        .blocklist_function("__acoshf64x")
        .blocklist_function("__acoshl")
        .blocklist_function("__acoshl")
        .blocklist_function("__acosl")
        .blocklist_function("__acosl")
        .blocklist_function("__asinf64x")
        .blocklist_function("__asinf64x")
        .blocklist_function("__asinhf64x")
        .blocklist_function("__asinhf64x")
        .blocklist_function("__asinhl")
        .blocklist_function("__asinhl")
        .blocklist_function("__asinl")
        .blocklist_function("__asinl")
        .blocklist_function("__atan2f64x")
        .blocklist_function("__atan2f64x")
        .blocklist_function("__atan2f64x")
        .blocklist_function("__atan2l")
        .blocklist_function("__atan2l")
        .blocklist_function("__atan2l")
        .blocklist_function("__atanf64x")
        .blocklist_function("__atanf64x")
        .blocklist_function("__atanhf64x")
        .blocklist_function("__atanhf64x")
        .blocklist_function("__atanhl")
        .blocklist_function("__atanhl")
        .blocklist_function("__atanl")
        .blocklist_function("__atanl")
        .blocklist_function("__cbrtf64x")
        .blocklist_function("__cbrtf64x")
        .blocklist_function("__cbrtl")
        .blocklist_function("__cbrtl")
        .blocklist_function("__ceilf64x")
        .blocklist_function("__ceilf64x")
        .blocklist_function("__ceill")
        .blocklist_function("__ceill")
        .blocklist_function("__copysignf64x")
        .blocklist_function("__copysignf64x")
        .blocklist_function("__copysignf64x")
        .blocklist_function("__copysignl")
        .blocklist_function("__copysignl")
        .blocklist_function("__copysignl")
        .blocklist_function("__cosf64x")
        .blocklist_function("__cosf64x")
        .blocklist_function("__coshf64x")
        .blocklist_function("__coshf64x")
        .blocklist_function("__coshl")
        .blocklist_function("__coshl")
        .blocklist_function("__cosl")
        .blocklist_function("__cosl")
        .blocklist_function("__dreml")
        .blocklist_function("__dreml")
        .blocklist_function("__dreml")
        .blocklist_function("__erfcf64x")
        .blocklist_function("__erfcf64x")
        .blocklist_function("__erfcl")
        .blocklist_function("__erfcl")
        .blocklist_function("__erff64x")
        .blocklist_function("__erff64x")
        .blocklist_function("__erfl")
        .blocklist_function("__erfl")
        .blocklist_function("__exp10f64x")
        .blocklist_function("__exp10f64x")
        .blocklist_function("__exp10l")
        .blocklist_function("__exp10l")
        .blocklist_function("__exp2f64x")
        .blocklist_function("__exp2f64x")
        .blocklist_function("__exp2l")
        .blocklist_function("__exp2l")
        .blocklist_function("__expf64x")
        .blocklist_function("__expf64x")
        .blocklist_function("__expl")
        .blocklist_function("__expl")
        .blocklist_function("__expm1f64x")
        .blocklist_function("__expm1f64x")
        .blocklist_function("__expm1l")
        .blocklist_function("__expm1l")
        .blocklist_function("__fabsf64x")
        .blocklist_function("__fabsf64x")
        .blocklist_function("__fabsl")
        .blocklist_function("__fabsl")
        .blocklist_function("__fdimf64x")
        .blocklist_function("__fdimf64x")
        .blocklist_function("__fdimf64x")
        .blocklist_function("__fdiml")
        .blocklist_function("__fdiml")
        .blocklist_function("__fdiml")
        .blocklist_function("__finitel")
        .blocklist_function("__floorf64x")
        .blocklist_function("__floorf64x")
        .blocklist_function("__floorl")
        .blocklist_function("__floorl")
        .blocklist_function("__fmaf64x")
        .blocklist_function("__fmaf64x")
        .blocklist_function("__fmaf64x")
        .blocklist_function("__fmaf64x")
        .blocklist_function("__fmal")
        .blocklist_function("__fmal")
        .blocklist_function("__fmal")
        .blocklist_function("__fmal")
        .blocklist_function("__fmaxf64x")
        .blocklist_function("__fmaxf64x")
        .blocklist_function("__fmaxf64x")
        .blocklist_function("__fmaxl")
        .blocklist_function("__fmaxl")
        .blocklist_function("__fmaxl")
        .blocklist_function("__fmaxmagf64x")
        .blocklist_function("__fmaxmagf64x")
        .blocklist_function("__fmaxmagf64x")
        .blocklist_function("__fmaxmagl")
        .blocklist_function("__fminf64x")
        .blocklist_function("__fminf64x")
        .blocklist_function("__fminf64x")
        .blocklist_function("__fminl")
        .blocklist_function("__fminl")
        .blocklist_function("__fminl")
        .blocklist_function("__fminmagf64x")
        .blocklist_function("__fminmagf64x")
        .blocklist_function("__fminmagf64x")
        .blocklist_function("__fminmagl")
        .blocklist_function("__fmodf64x")
        .blocklist_function("__fmodf64x")
        .blocklist_function("__fmodf64x")
        .blocklist_function("__fmodl")
        .blocklist_function("__fmodl")
        .blocklist_function("__fmodl")
        .blocklist_function("__fpclassifyl")
        .blocklist_function("__frexpf64x")
        .blocklist_function("__frexpf64x")
        .blocklist_function("__frexpl")
        .blocklist_function("__frexpl")
        .blocklist_function("__fromfpf64x")
        .blocklist_function("__fromfpl")
        .blocklist_function("__fromfpxf64x")
        .blocklist_function("__fromfpxl")
        .blocklist_function("__gammal")
        .blocklist_function("__gammal")
        .blocklist_function("__getpayloadf64x")
        .blocklist_function("__getpayloadf64x")
        .blocklist_function("__getpayloadl")
        .blocklist_function("__hypotf64x")
        .blocklist_function("__hypotf64x")
        .blocklist_function("__hypotf64x")
        .blocklist_function("__hypotl")
        .blocklist_function("__hypotl")
        .blocklist_function("__hypotl")
        .blocklist_function("__ilogbf64x")
        .blocklist_function("__ilogbl")
        .blocklist_function("__iscanonicall")
        .blocklist_function("__iseqsigl")
        .blocklist_function("__iseqsigl")
        .blocklist_function("__isinfl")
        .blocklist_function("__isnanl")
        .blocklist_function("__issignalingl")
        .blocklist_function("__j0f64x")
        .blocklist_function("__j0f64x")
        .blocklist_function("__j0l")
        .blocklist_function("__j0l")
        .blocklist_function("__j1f64x")
        .blocklist_function("__j1f64x")
        .blocklist_function("__j1l")
        .blocklist_function("__j1l")
        .blocklist_function("__jnf64x")
        .blocklist_function("__jnf64x")
        .blocklist_function("__jnl")
        .blocklist_function("__jnl")
        .blocklist_function("__ldexpf64x")
        .blocklist_function("__ldexpf64x")
        .blocklist_function("__ldexpl")
        .blocklist_function("__ldexpl")
        .blocklist_function("__lgammaf64x")
        .blocklist_function("__lgammaf64x")
        .blocklist_function("__lgammaf64x_r")
        .blocklist_function("__lgammaf64x_r")
        .blocklist_function("__lgammal")
        .blocklist_function("__lgammal")
        .blocklist_function("__lgammal_r")
        .blocklist_function("__lgammal_r")
        .blocklist_function("__llogbf64x")
        .blocklist_function("__llogbl")
        .blocklist_function("__llrintf64x")
        .blocklist_function("__llrintl")
        .blocklist_function("__llroundf64x")
        .blocklist_function("__llroundl")
        .blocklist_function("__log10f64x")
        .blocklist_function("__log10f64x")
        .blocklist_function("__log10l")
        .blocklist_function("__log10l")
        .blocklist_function("__log1pf64x")
        .blocklist_function("__log1pf64x")
        .blocklist_function("__log1pl")
        .blocklist_function("__log1pl")
        .blocklist_function("__log2f64x")
        .blocklist_function("__log2f64x")
        .blocklist_function("__log2l")
        .blocklist_function("__log2l")
        .blocklist_function("__logbf64x")
        .blocklist_function("__logbf64x")
        .blocklist_function("__logbl")
        .blocklist_function("__logbl")
        .blocklist_function("__logf64x")
        .blocklist_function("__logf64x")
        .blocklist_function("__logl")
        .blocklist_function("__logl")
        .blocklist_function("__lrintf64x")
        .blocklist_function("__lrintl")
        .blocklist_function("__lroundf64x")
        .blocklist_function("__lroundl")
        .blocklist_function("__modff64x")
        .blocklist_function("__modff64x")
        .blocklist_function("__modff64x")
        .blocklist_function("__modfl")
        .blocklist_function("__modfl")
        .blocklist_function("__modfl")
        .blocklist_function("__nanf64x")
        .blocklist_function("__nanl")
        .blocklist_function("__nearbyintf64x")
        .blocklist_function("__nearbyintf64x")
        .blocklist_function("__nearbyintl")
        .blocklist_function("__nearbyintl")
        .blocklist_function("__nextafterf64x")
        .blocklist_function("__nextafterf64x")
        .blocklist_function("__nextafterf64x")
        .blocklist_function("__nextafterl")
        .blocklist_function("__nextafterl")
        .blocklist_function("__nextafterl")
        .blocklist_function("__nextdownf64x")
        .blocklist_function("__nextdownf64x")
        .blocklist_function("__nextdownl")
        .blocklist_function("__nextdownl")
        .blocklist_function("__nexttoward")
        .blocklist_function("__nexttowardf")
        .blocklist_function("__nexttowardl")
        .blocklist_function("__nexttowardl")
        .blocklist_function("__nexttowardl")
        .blocklist_function("__nextupf64x")
        .blocklist_function("__nextupf64x")
        .blocklist_function("__nextupl")
        .blocklist_function("__nextupl")
        .blocklist_function("__powf64x")
        .blocklist_function("__powf64x")
        .blocklist_function("__powf64x")
        .blocklist_function("__powl")
        .blocklist_function("__powl")
        .blocklist_function("__powl")
        .blocklist_function("__remainderf64x")
        .blocklist_function("__remainderf64x")
        .blocklist_function("__remainderf64x")
        .blocklist_function("__remainderl")
        .blocklist_function("__remainderl")
        .blocklist_function("__remainderl")
        .blocklist_function("__remquof64x")
        .blocklist_function("__remquol")
        .blocklist_function("__remquol")
        .blocklist_function("__remquol")
        .blocklist_function("__rintf64x")
        .blocklist_function("__rintf64x")
        .blocklist_function("__rintl")
        .blocklist_function("__rintl")
        .blocklist_function("__roundevenf64x")
        .blocklist_function("__roundevenf64x")
        .blocklist_function("__roundevenl")
        .blocklist_function("__roundevenl")
        .blocklist_function("__roundf64x")
        .blocklist_function("__roundf64x")
        .blocklist_function("__roundl")
        .blocklist_function("__roundl")
        .blocklist_function("__scalbl")
        .blocklist_function("__scalblnf64x")
        .blocklist_function("__scalblnf64x")
        .blocklist_function("__scalblnl")
        .blocklist_function("__scalblnl")
        .blocklist_function("__scalbnf64x")
        .blocklist_function("__scalbnf64x")
        .blocklist_function("__scalbnl")
        .blocklist_function("__scalbnl")
        .blocklist_function("__signbitl")
        .blocklist_function("__significandl")
        .blocklist_function("__significandl")
        .blocklist_function("__sincosf64x")
        .blocklist_function("__sincosf64x")
        .blocklist_function("__sincosf64x")
        .blocklist_function("__sincosl")
        .blocklist_function("__sincosl")
        .blocklist_function("__sincosl")
        .blocklist_function("__sinf64x")
        .blocklist_function("__sinf64x")
        .blocklist_function("__sinhf64x")
        .blocklist_function("__sinhf64x")
        .blocklist_function("__sinhl")
        .blocklist_function("__sinhl")
        .blocklist_function("__sinl")
        .blocklist_function("__sinl")
        .blocklist_function("__sqrtf64x")
        .blocklist_function("__sqrtf64x")
        .blocklist_function("__sqrtl")
        .blocklist_function("__sqrtl")
        .blocklist_function("__tanf64x")
        .blocklist_function("__tanf64x")
        .blocklist_function("__tanhf64x")
        .blocklist_function("__tanhf64x")
        .blocklist_function("__tanhl")
        .blocklist_function("__tanhl")
        .blocklist_function("__tanl")
        .blocklist_function("__tanl")
        .blocklist_function("__tgammaf64x")
        .blocklist_function("__tgammaf64x")
        .blocklist_function("__tgammal")
        .blocklist_function("__tgammal")
        .blocklist_function("__truncf64x")
        .blocklist_function("__truncf64x")
        .blocklist_function("__truncl")
        .blocklist_function("__truncl")
        .blocklist_function("__ufromfpf64x")
        .blocklist_function("__ufromfpl")
        .blocklist_function("__ufromfpxf64x")
        .blocklist_function("__ufromfpxl")
        .blocklist_function("__y0f64x")
        .blocklist_function("__y0f64x")
        .blocklist_function("__y0l")
        .blocklist_function("__y0l")
        .blocklist_function("__y1f64x")
        .blocklist_function("__y1f64x")
        .blocklist_function("__y1l")
        .blocklist_function("__y1l")
        .blocklist_function("__ynf64x")
        .blocklist_function("__ynf64x")
        .blocklist_function("__ynl")
        .blocklist_function("__ynl")
        .blocklist_function("acosf64x")
        .blocklist_function("acosf64x")
        .blocklist_function("acoshf64x")
        .blocklist_function("acoshf64x")
        .blocklist_function("acoshl")
        .blocklist_function("acoshl")
        .blocklist_function("acosl")
        .blocklist_function("acosl")
        .blocklist_function("asinf64x")
        .blocklist_function("asinf64x")
        .blocklist_function("asinhf64x")
        .blocklist_function("asinhf64x")
        .blocklist_function("asinhl")
        .blocklist_function("asinhl")
        .blocklist_function("asinl")
        .blocklist_function("asinl")
        .blocklist_function("atan2f64x")
        .blocklist_function("atan2f64x")
        .blocklist_function("atan2f64x")
        .blocklist_function("atan2l")
        .blocklist_function("atan2l")
        .blocklist_function("atan2l")
        .blocklist_function("atanf64x")
        .blocklist_function("atanf64x")
        .blocklist_function("atanhf64x")
        .blocklist_function("atanhf64x")
        .blocklist_function("atanhl")
        .blocklist_function("atanhl")
        .blocklist_function("atanl")
        .blocklist_function("atanl")
        .blocklist_function("canonicalizef64x")
        .blocklist_function("canonicalizef64x")
        .blocklist_function("canonicalizel")
        .blocklist_function("cbrtf64x")
        .blocklist_function("cbrtf64x")
        .blocklist_function("cbrtl")
        .blocklist_function("cbrtl")
        .blocklist_function("ceilf64x")
        .blocklist_function("ceilf64x")
        .blocklist_function("ceill")
        .blocklist_function("ceill")
        .blocklist_function("clock_adjtime")
        .blocklist_function("copysignf64x")
        .blocklist_function("copysignf64x")
        .blocklist_function("copysignf64x")
        .blocklist_function("copysignl")
        .blocklist_function("copysignl")
        .blocklist_function("copysignl")
        .blocklist_function("cosf64x")
        .blocklist_function("cosf64x")
        .blocklist_function("coshf64x")
        .blocklist_function("coshf64x")
        .blocklist_function("coshl")
        .blocklist_function("coshl")
        .blocklist_function("cosl")
        .blocklist_function("cosl")
        .blocklist_function("daddl")
        .blocklist_function("ddivl")
        .blocklist_function("dmull")
        .blocklist_function("dreml")
        .blocklist_function("dreml")
        .blocklist_function("dreml")
        .blocklist_function("dsubl")
        .blocklist_function("erfcf64x")
        .blocklist_function("erfcf64x")
        .blocklist_function("erfcl")
        .blocklist_function("erfcl")
        .blocklist_function("erff64x")
        .blocklist_function("erff64x")
        .blocklist_function("erfl")
        .blocklist_function("erfl")
        .blocklist_function("exp10f64x")
        .blocklist_function("exp10f64x")
        .blocklist_function("exp10l")
        .blocklist_function("exp10l")
        .blocklist_function("exp2f64x")
        .blocklist_function("exp2f64x")
        .blocklist_function("exp2l")
        .blocklist_function("exp2l")
        .blocklist_function("expf64x")
        .blocklist_function("expf64x")
        .blocklist_function("expl")
        .blocklist_function("expl")
        .blocklist_function("expm1f64x")
        .blocklist_function("expm1f64x")
        .blocklist_function("expm1l")
        .blocklist_function("expm1l")
        .blocklist_function("exttoward")
        .blocklist_function("f32addf64x")
        .blocklist_function("f32divf64x")
        .blocklist_function("f32mulf64x")
        .blocklist_function("f32subf64x")
        .blocklist_function("f32xaddf64x")
        .blocklist_function("f32xdivf64x")
        .blocklist_function("f32xmulf64x")
        .blocklist_function("f32xsubf64x")
        .blocklist_function("f64addf64x")
        .blocklist_function("f64divf64x")
        .blocklist_function("f64mulf64x")
        .blocklist_function("f64subf64x")
        .blocklist_function("fabsf64x")
        .blocklist_function("fabsf64x")
        .blocklist_function("fabsl")
        .blocklist_function("fabsl")
        .blocklist_function("faddl")
        .blocklist_function("fdimf64x")
        .blocklist_function("fdimf64x")
        .blocklist_function("fdimf64x")
        .blocklist_function("fdiml")
        .blocklist_function("fdiml")
        .blocklist_function("fdiml")
        .blocklist_function("fdivl")
        .blocklist_function("finitel")
        .blocklist_function("floorf64x")
        .blocklist_function("floorf64x")
        .blocklist_function("floorl")
        .blocklist_function("floorl")
        .blocklist_function("fmaf64x")
        .blocklist_function("fmaf64x")
        .blocklist_function("fmaf64x")
        .blocklist_function("fmaf64x")
        .blocklist_function("fmal")
        .blocklist_function("fmal")
        .blocklist_function("fmal")
        .blocklist_function("fmal")
        .blocklist_function("fmaxf64x")
        .blocklist_function("fmaxf64x")
        .blocklist_function("fmaxf64x")
        .blocklist_function("fmaxl")
        .blocklist_function("fmaxl")
        .blocklist_function("fmaxl")
        .blocklist_function("fmaxmagf64x")
        .blocklist_function("fmaxmagf64x")
        .blocklist_function("fmaxmagf64x")
        .blocklist_function("fmaxmagl")
        .blocklist_function("fminf64x")
        .blocklist_function("fminf64x")
        .blocklist_function("fminf64x")
        .blocklist_function("fminl")
        .blocklist_function("fminl")
        .blocklist_function("fminl")
        .blocklist_function("fminmagf64x")
        .blocklist_function("fminmagf64x")
        .blocklist_function("fminmagf64x")
        .blocklist_function("fminmagl")
        .blocklist_function("fmodf64x")
        .blocklist_function("fmodf64x")
        .blocklist_function("fmodf64x")
        .blocklist_function("fmodl")
        .blocklist_function("fmodl")
        .blocklist_function("fmodl")
        .blocklist_function("fmull")
        .blocklist_function("frexpf64x")
        .blocklist_function("frexpf64x")
        .blocklist_function("frexpl")
        .blocklist_function("frexpl")
        .blocklist_function("fromfpf64x")
        .blocklist_function("fromfpl")
        .blocklist_function("fromfpxf64x")
        .blocklist_function("fromfpxl")
        .blocklist_function("fsubl")
        .blocklist_function("gammal")
        .blocklist_function("gammal")
        .blocklist_function("getpayloadf64x")
        .blocklist_function("getpayloadf64x")
        .blocklist_function("getpayloadl")
        .blocklist_function("hypotf64x")
        .blocklist_function("hypotf64x")
        .blocklist_function("hypotf64x")
        .blocklist_function("hypotl")
        .blocklist_function("hypotl")
        .blocklist_function("hypotl")
        .blocklist_function("ilogbf64x")
        .blocklist_function("ilogbl")
        .blocklist_function("isinfl")
        .blocklist_function("isnanl")
        .blocklist_function("j0f64x")
        .blocklist_function("j0f64x")
        .blocklist_function("j0l")
        .blocklist_function("j0l")
        .blocklist_function("j1f64x")
        .blocklist_function("j1f64x")
        .blocklist_function("j1l")
        .blocklist_function("j1l")
        .blocklist_function("jnf64x")
        .blocklist_function("jnf64x")
        .blocklist_function("jnl")
        .blocklist_function("jnl")
        .blocklist_function("ldexpf64x")
        .blocklist_function("ldexpf64x")
        .blocklist_function("ldexpl")
        .blocklist_function("ldexpl")
        .blocklist_function("lgammaf64x")
        .blocklist_function("lgammaf64x")
        .blocklist_function("lgammaf64x_r")
        .blocklist_function("lgammaf64x_r")
        .blocklist_function("lgammal")
        .blocklist_function("lgammal")
        .blocklist_function("lgammal_r")
        .blocklist_function("lgammal_r")
        .blocklist_function("llogbf64x")
        .blocklist_function("llogbl")
        .blocklist_function("llrintf64x")
        .blocklist_function("llrintl")
        .blocklist_function("llroundf64x")
        .blocklist_function("llroundl")
        .blocklist_function("log10f64x")
        .blocklist_function("log10f64x")
        .blocklist_function("log10l")
        .blocklist_function("log10l")
        .blocklist_function("log1pf64x")
        .blocklist_function("log1pf64x")
        .blocklist_function("log1pl")
        .blocklist_function("log1pl")
        .blocklist_function("log2f64x")
        .blocklist_function("log2f64x")
        .blocklist_function("log2l")
        .blocklist_function("log2l")
        .blocklist_function("logbf64x")
        .blocklist_function("logbf64x")
        .blocklist_function("logbl")
        .blocklist_function("logbl")
        .blocklist_function("logf64x")
        .blocklist_function("logf64x")
        .blocklist_function("logl")
        .blocklist_function("logl")
        .blocklist_function("lrintf64x")
        .blocklist_function("lrintl")
        .blocklist_function("lroundf64x")
        .blocklist_function("lroundl")
        .blocklist_function("modff64x")
        .blocklist_function("modff64x")
        .blocklist_function("modff64x")
        .blocklist_function("modfl")
        .blocklist_function("modfl")
        .blocklist_function("modfl")
        .blocklist_function("nanf64x")
        .blocklist_function("nanl")
        .blocklist_function("nearbyintf64x")
        .blocklist_function("nearbyintf64x")
        .blocklist_function("nearbyintl")
        .blocklist_function("nearbyintl")
        .blocklist_function("nextafterf64x")
        .blocklist_function("nextafterf64x")
        .blocklist_function("nextafterf64x")
        .blocklist_function("nextafterl")
        .blocklist_function("nextafterl")
        .blocklist_function("nextafterl")
        .blocklist_function("nextdownf64x")
        .blocklist_function("nextdownf64x")
        .blocklist_function("nextdownl")
        .blocklist_function("nextdownl")
        .blocklist_function("nexttoward")
        .blocklist_function("nexttowardf")
        .blocklist_function("nexttowardl")
        .blocklist_function("nexttowardl")
        .blocklist_function("nexttowardl")
        .blocklist_function("nextupf64x")
        .blocklist_function("nextupf64x")
        .blocklist_function("nextupl")
        .blocklist_function("nextupl")
        .blocklist_function("powf64x")
        .blocklist_function("powf64x")
        .blocklist_function("powf64x")
        .blocklist_function("powl")
        .blocklist_function("powl")
        .blocklist_function("powl")
        .blocklist_function("qecvt")
        .blocklist_function("qecvt_r")
        .blocklist_function("qfcvt")
        .blocklist_function("qfcvt_r")
        .blocklist_function("qgcvt")
        .blocklist_function("remainderf64x")
        .blocklist_function("remainderf64x")
        .blocklist_function("remainderf64x")
        .blocklist_function("remainderl")
        .blocklist_function("remainderl")
        .blocklist_function("remainderl")
        .blocklist_function("remquof64x")
        .blocklist_function("remquol")
        .blocklist_function("remquol")
        .blocklist_function("remquol")
        .blocklist_function("rintf64x")
        .blocklist_function("rintf64x")
        .blocklist_function("rintl")
        .blocklist_function("rintl")
        .blocklist_function("roundevenf64x")
        .blocklist_function("roundevenf64x")
        .blocklist_function("roundevenl")
        .blocklist_function("roundevenl")
        .blocklist_function("roundf64x")
        .blocklist_function("roundf64x")
        .blocklist_function("roundl")
        .blocklist_function("roundl")
        .blocklist_function("scalbl")
        .blocklist_function("scalblnf64x")
        .blocklist_function("scalblnf64x")
        .blocklist_function("scalblnl")
        .blocklist_function("scalblnl")
        .blocklist_function("scalbnf64x")
        .blocklist_function("scalbnf64x")
        .blocklist_function("scalbnl")
        .blocklist_function("scalbnl")
        .blocklist_function("setpayloadf64x")
        .blocklist_function("setpayloadf64x")
        .blocklist_function("setpayloadl")
        .blocklist_function("setpayloadsigf64x")
        .blocklist_function("setpayloadsigf64x")
        .blocklist_function("setpayloadsigl")
        .blocklist_function("significandl")
        .blocklist_function("significandl")
        .blocklist_function("sincosf64x")
        .blocklist_function("sincosf64x")
        .blocklist_function("sincosf64x")
        .blocklist_function("sincosl")
        .blocklist_function("sincosl")
        .blocklist_function("sincosl")
        .blocklist_function("sinf64x")
        .blocklist_function("sinf64x")
        .blocklist_function("sinhf64x")
        .blocklist_function("sinhf64x")
        .blocklist_function("sinhl")
        .blocklist_function("sinhl")
        .blocklist_function("sinl")
        .blocklist_function("sinl")
        .blocklist_function("sqrtf64x")
        .blocklist_function("sqrtf64x")
        .blocklist_function("sqrtl")
        .blocklist_function("sqrtl")
        .blocklist_function("strfromf64x")
        .blocklist_function("strfroml")
        .blocklist_function("strtof64x")
        .blocklist_function("strtof64x_l")
        .blocklist_function("strtold")
        .blocklist_function("strtold_l")
        .blocklist_function("tanf64x")
        .blocklist_function("tanf64x")
        .blocklist_function("tanhf64x")
        .blocklist_function("tanhf64x")
        .blocklist_function("tanhl")
        .blocklist_function("tanhl")
        .blocklist_function("tanl")
        .blocklist_function("tanl")
        .blocklist_function("tgammaf64x")
        .blocklist_function("tgammaf64x")
        .blocklist_function("tgammal")
        .blocklist_function("tgammal")
        .blocklist_function("totalorderf64x")
        .blocklist_function("totalorderf64x")
        .blocklist_function("totalorderl")
        .blocklist_function("totalordermagf64x")
        .blocklist_function("totalordermagf64x")
        .blocklist_function("totalordermagl")
        .blocklist_function("truncf64x")
        .blocklist_function("truncf64x")
        .blocklist_function("truncl")
        .blocklist_function("truncl")
        .blocklist_function("ufromfpf64x")
        .blocklist_function("ufromfpl")
        .blocklist_function("ufromfpxf64x")
        .blocklist_function("ufromfpxl")
        .blocklist_function("y0f64x")
        .blocklist_function("y0f64x")
        .blocklist_function("y0l")
        .blocklist_function("y0l")
        .blocklist_function("y1f64x")
        .blocklist_function("y1f64x")
        .blocklist_function("y1l")
        .blocklist_function("y1l")
        .blocklist_function("ynf64x")
        .blocklist_function("ynf64x")
        .blocklist_function("ynl")
        .blocklist_function("ynl")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_INT_DOWNWARD")
        .blocklist_item("FP_INT_TONEAREST")
        .blocklist_item("FP_INT_TONEARESTFROMZERO")
        .blocklist_item("FP_INT_TOWARDZERO")
        .blocklist_item("FP_INT_UPWARD")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_ZERO")
        .blocklist_type("_Float64x")
        .blocklist_type("timex")
        .generate()
        .expect("Unable to generate bindings");

    let generated_path = out_path.join("php_bindings.rs");
    bindings
        .write_to_file(&generated_path)
        .expect("Unable to write output file");
}

fn execute_command<S: AsRef<OsStr> + Debug>(argv: &[S]) -> String {
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]);
    let output = command
        .output()
        .unwrap_or_else(|_| panic!("Execute command {:?} failed", &argv))
        .stdout;
    String::from_utf8(output).unwrap().trim().to_owned()
}
