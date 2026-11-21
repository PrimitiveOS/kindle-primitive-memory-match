#!/bin/sh
# Name: Memory Match
# Author: Primitive AI
# Icon: data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAAEACAAAAAB5Gfe6AAAQ7ElEQVR4Ae3AA6AkWZbG8f937o3IzKdyS2Oubdu2bdu2bdu2bWmMnpZKr54yMyLu+Xa3anqmhztr1U/m/zVk/l9D5v81ZP5fQ+b/NWT+X0Pm/zVk/l9D5v81ZP5fQ+b/NWT+X0Pm/zVk/l9D5v81ZP5fQ+b/NWT+X0Pm/zVk/l9D5v81ZP5fQ+b/NWT+X0Pm/zVk/l9D5r/G1/z9E36f/yqv/ugX/yheJMj8V/jsz+G/2md9Ni8CZP7z/eKb8d/hF96UfxEy/+l+8c347/ELb8q/BJn/bL/4Zvx3+YU35V+AzH828d/H/AuQ+U/22Z/Df5/P+mxeOGT+k4n/TuaFQ+Y/19d8NP+dvvqjeKGQ+c/1Ad/Of6f3/zZeKGT+c73G7/Pf6dV/jxcKmf9c4r+XeaGQ+c8l/nuZFwqZ/1ziv5d5oZD5zyX+e5kXCpn/XOK/l3mhkPnPJf57mRcKmf9c4r+XeaGQ+c8l/nuZFwqZ/1ziv5d5oZD5zyX+e5kXCpn/XOK/l3mhkPnPJf57mRcKmf9c4r+XeaGQ+c8l/nuZFwqZ/1ziv5d5oZD5zyX+e5kXCpkXwdf8/RN+n/+dzAuFzL/osz+H/73MC4XMv+AX34z/zcwLhcwL94tvxv9q5oVC5oX6xTfjfzfzQiHzQgkAPvWV34J/G/Hfy7xQyLwwn/05AK/7G/ybif9e5oVC5oURwOv+Bv924r+XeaGQeSG+5qMBzL+D+O9lXihkXogP+HbgU7+Afwfx38u8UMi8EK/x+8DPvgX/DuK/l3mhkHkhBGD+PcR/L/NCIfNCCMD8e4j/XuaFQuaFEID59xD/vcwLhcwLIQDz7yH+e5kXCpkXQgDm30P89zIvFDIvhADMv4f472VeKGReCAGYfw/x7/Pu38+/i3mhkHkhBGD+PcS/j8W/i3mhkHkhBGD+PcS/y5d84pd+Ev8e5oVC5oUQgPn3EP8uBvHvYV4oZF4IAZh/D/Hv8f7fBh/w7fw7mBcKmRdCAObfQ/x7/M1Lwt++FP8O5oVC5oUQgPn3EP8Or/1bAK/z2/zbmRcKmRdCAObfQ/w7/MTbAvzk2/FvZ14oZF4IAZh/D/HvYC4T/3bmhULmhRCA+fcQ/3Zf8olc9qWfxL+ZeaGQeSEEYP49xL+deSbxb2ZeKGReCAGYfw/xb/b+38YzfcC3829lXihkXggBmH8P8W/2Ny/JM/3tS/FvZV4oZF4IAZh/D/Fv9dq/xbO8zm/zb2ReKGReCAGYf4M/PXf6FQEQ/1Y/8bY8y0++Hf9G5oVC5oUQgPnXe4ufhzf/OQDxb2UeQPwbmRcKmRdCAOZfTVxmQPwbfckn8gBf+kn825gXCpkXQgDmX0s8k0H8G5nnIP5tzAuFzAshAPOvJJ7FiH+b9/82nsMHfDv/JuaFQuaFEID51xEPYPFv8zcvyXP425fi38S8UMi8EAIw/yriP8Jr/xbP5XV+m38L80Ih80IIwPxriP8QP/G2PJeffDv+LcwLhcwLIQDzryD+Y5jnIf4tzAuFzAshAPOiE/8xvuQTeR5f+kn8G5gXCpkXQgDmRSbu96a/8Ga/yL+deT7Ev4F5oZB5IQRgXlTifm/6C/Bmv8i/1ft/G8/HB3w7/3rmhULmhRCAeRGJ+73pLwC82S/yb/Q3L8nz8bcvxb+eeaGQeSEEYF404n5v+gtc9ma/yL/Ja/8Wz9fr/Db/auaFQuaFEIB5kYj7vekv8Exv9os8P+/xurxQL//iPF9//+e8UL/5fTwP80Ih80IIwLwoxP3e9Bd4ljf7RZ6fb/pg/hN884fwvMwLhcwLIQDzIhD3e9Nf4AHe7Bd5fj786/gP9xFfz/NhXihkXggBmH+ZuN+b/gLPQTxfb/Cr/Ad7w1/j+TEvFDIvhADMv0jc701/gecknr9H/dwj+A/05Ld4Is+XeaGQeSEEYP4l4n5v+gtc8VvnT70OAOIF+fXX4z/Mb7w+L4B5oZB5IQRg/gXifm/6CwDA6/0mvO5vAIgX6Js+mP8g3/whvCDmhULmhRCAeeHE/d70F7jsKY/gsic/HMQL9uFfx3+Ij/h6XiDzQiHzQgjAvFDifm/6C1z2lEfwTE9+OOKFeINf5T/AG/4aL5h5oZB5IQRgXhhxvzf9BS57yiN4lic/XLwwj/q5R/Dv9OS3eCIvhHmhkHkhBGBeCHG/N/0FLnvKI3iAJz+CF+7XX49/l994fV4o80Ih80IIwLxg4n5v+gtc9pRH8K/zTR/Mv8M3fwgvnHmhkHkhBGBeIHG/N/0FLnvKI/jX+vCv49/sI76ef4F5oZB5IQRgXhBxvzf9BQB4yiP413uDX+Xf6A1/jX+JeaGQeSEEYF4Acb83/QUAeMoj+Ld41M89gn+DJ7/FE/kXmRcKmRdCAOb5E/d701/gsqc8gvu93Y+//U/wovv11+Nf7TdenxeBeaGQeSEEYJ4vcb83/QUue8ojuN/b/Ti8/U/wovumD+Zf6Zs/hBeFeaGQeSEEYJ4fcb83/QUue8ojuN/b/TjA2/8EL7oP/zr+VT7i63mRmBcKmRdCAOb5EPd701/gsqc8gvu93Y9z2dv/BC+6N/hV/hXe8Nd40ZgXCpkXQgDmeYn7vekvcNlTHsH93u7Heaa3/wledI/6uUfwInryWzyRF5F5oZB5IQRgnoe435v+Apc95RHc7+1+nGd5+5/gX8G8iMSLzLxQyLwQAjDPTdzvTX+BK8T93u7HeYC3/wledOZFJF5k5oVC5oUQgHkub/HzPNOb/gIA8Hq/yTO93Y/zHMSL7KX/ihfRy/w1LyrzQiHzQgjAPKc/fSWe6U1/gSt+63V5prf7cZ6TeJF99FfxIvqYr+ZFZV4oZF4IAZjn9ItvxjOZZ/rxd+CZzHMRL7KfemteRD/9NryozAuFzAshAPOc/vSVeKY3/QWu+K3X5Zne7sd5TuJFZl5k4kVlXihkXggBmOfyFj/PM73pLwAAr/ebPNPb/TjPQbyoXvqveJG9zF/zIjIvFDIvhADMcxP3e9Nf4Apxv7f7cR7g7X+CF9VHfxUvso/5al5E5oVC5oUQgHke4n5v+gtc9pRHcL+3+3Ge5e1/ghfZT701L7KffhteROaFQuaFEIB5XuJ+b/oLXPaUR3C/t/txnuntf4IXnflXEC8i80Ih80IIwDwf4n5v+gtc9pRHcL+3+3Eue/uf4EX30n/Fv8LL/DUvGvNCIfNCCMA8P+J+b/oLXPaUR3C/t/txgLf/Cf4VPvqr+Ff4mK/mRWNeKGReCAGY50vc701/gcue8gju93Y/Dm//E/xr/NRb86/w02/Di8a8UMi8EAIwz5+435v+Apc95RHc7+1+/O1/gn8V8/x8xNd/+Nfx/IgXjXmhkHkhBGBeAHG/N/0FAHjKI/i3eum/4nn97bv9Pbz4D7wkz+tl/poXiXmhkHkhBGBeEHG/N/0FAHjKI/g3+uiv4nl8zUdz2Vd/FM/jY76aF4l5oZB5IQRgXiBxvzf9BS57yiP4t/mpt+a5vckv80xv/Es8t59+G14k5oVC5oUQgHnBxP3e9Be47CmP4N/EPJdfeHMe4OffjOciXiTmhULmhRCAeSHE/d70F7jsKY/gAZ78CF4kL/1XPKeP+Hqew4d/Hc/pZf6aF4V5oZB5IQRgXhhxvzf9BS57yiN4lic/XLxIPvqreKC/fbe/57m8+A+8JA/0MV/Ni8K8UMi8EAIwL5S435v+Apc95RE805MfjniR/NRb8wBf89E8H1/9UTzAT78NLwrzQiHzQgjAvHDifm/6C1z2lEdw2ZMfDuJFYh7gTX6Z5+uNf4kHEC8K80Ih80IIwPwLxP3e9BcAgNf7TXjd3wAQL4qX/iue5RfenBfo59+MZ3mZv+ZFYF4oZF4IAZh/ibjfm/4CV/zW+VOvA4B4UXz0V3G/j/h6XogP/zru9zFfzYvAvFDIvBACMP8icb83/QWek3hR/NRbc8Xfvtvf80K9+A+8JFf89NvwIjAvFDIvhADMv0zc701/gecgXhQGAL7mo/kXffVHcYV4EZgXCpkXQgDmRSDu96a/wAO82S/yInjpvwKAN/llXgRv/Etc9jJ/zb/MvFDIvBACMC8Kcb83/QWe5c1+kRfFR38VwC+8OS+in38zgI/5av5l5oVC5oUQgHmRiPu96S/wTG/2i7xIfuqtgY/4el5kH/51wE+/Df8y80Ih80IIwLxoxP3e9Be47M1+kReN4W/f7e/5V3jxH3hJEP8y80Ih80IIwLyIxP3e9BcA3uwXedG89F/xNR/Nv9JXfxQv89f8i8wLhcwLIQDzohL3e9NfgDf7RV5EH/1Vb/LL/Ku98S99zFfzLzIvFDIvhADMi0zc701/4c1+kRfVB3wb/yYf8G38i8wLhcwLIQDzohP/05gXCpkXQgDmX0H8D2NeKGReCAGYfw3xP4t5oZB5IQRg/lXE/yjmhULmhRCA+dcRD2Dx38u8UMi8EAIw/0riWYz472VeKGReCAGYfy3xTAbx38u8UMi8EAIw/2riMgPiv5d5oZB5IQRg/vXe4ufhzX8OQPz3Mi8UMi+EAMy/wZ+eO/2KAIj/XuaFQuaFEID59xD/vcwLhcwLIQDz7yH+e5kXCpkXQgDm30P89zIvFDIvhADMv4f472VeKGReCAGYfw/x38u8UMi8EAIw/x7iv5d5oZB5IQRg/j3Efy/zQiHzQgjA/HuI/17mhULmhRCA+fcQ/73MC4XMCyEA8+8h/nuZFwqZF0IA5t9D/PcyLxQyL4QAzL+H+O9lXihkXggBmH8P8d/LvFDIvBACMP8e4r+XeaGQeSEEYP49xH8v80Ih80IIwPx7iP9e5oVC5oUQgPn3EP+9zAuFzAshAPPvIf57mRcKmRdCAObfQ/z3Mi8UMi+EAMy/x2v8Pv+dXv33eKGQeSEEYP49PuDb+e/0/t/GC4XMCyEA8+/xNR/Nf6ev/iheKGReCAGYfxfx38m8cMi8EAIw/y6f/Tn89/msz+aFQ+aFEID59xH/fcy/AJkXQgDm3+cX34z/Lr/wpvwLkHkhBGD+nX7xzfjv8Qtvyr8EmRdCAObf6xffjP8Ov/Cm/IuQeSEEYP79Pvtz+K/2WZ/NiwCZF0IA5j/C1/z9E36f/yqv/ugX/yheJMi8EAIw/+UEYP4LIPNCCMD8lxOA+S+AzAshAPNfTgDmvwAyL4QAzH85AZj/Asi8EAIw/+UEYP4LIPNCCMD8lxOA+S+AzAshAPNfTgDmvwAyL4QAzH85AZj/Asi8EAIw/+UEYP4LIPNCCMD8lxOA+S+AzAshAPNfTgDmvwAyL4QAzH85AZj/Asi8EAIw/+UEYP4LIPNCCMD8lxOA+S+AzAvxGr8P/Oxb8F/s594SePXf478AMi/EB3w78KlfwH+xT/tC4P2/jf8CyLwQX/PRAOa/mAC++qP4L4DMCyOA1/0N/ku93m8CmP8KyLwwn/05AK/7G/wXer3fBPisz+a/AjIvlADgU1/5Lfgv8XN//IVcZv5LIPNC/eKb8d/iF96U/xLIvHC/+Gb8N/iFN+W/BjL/gl98M/7L/cKb8l8EmX/RZ38O/6U+67P5L4PMi+Br/v4Jv89/iVd/9It/FP+FkPl/DZn/15D5fw2Z/9eQ+X8Nmf/XkPl/DZn/15D5fw2Z/9eQ+X8Nmf/XkPl/DZn/15D5fw2Z/9eQ+X8Nmf/XkPl/DZn/15D5fw2Z/9eQ+X8Nmf/XkPl/DZn/15D5fw2Z/9eQ+X8Nmf/XkPl/DZn/15D5fw2Z/9eQ+X8Nmf/XkPl/DZn/15D5fw2Z/9f4R48O9y6BoM8sAAAAAElFTkSuQmCC
# DontUseFBInk
# PrimitiveUSBLease
# Wake: redraw

# Under the launcher SIGUSR1 asks the app to redraw: this script ignores it (a shell ends on it otherwise); the
# app sets its own handler.
[ "${PRIMITIVE_LAUNCHER:-0}" = 1 ] && trap '' USR1
app=/mnt/us/memorypairs/bin/memorypairs-kindle
[ -x "$app" ] || exit 1
if [ -n "${KL_NATIVE_APP_RUN:-}" ]; then
    cp "$app" "$KL_NATIVE_APP_RUN/run" && chmod 755 "$KL_NATIVE_APP_RUN/run" || exit 1
    cd / || exit 1
    exec "$KL_NATIVE_APP_RUN/run"
fi

# Older launchers and the stock library start this entry without a shared RAM work directory.
case "$0" in
    /var/tmp/primitive-memorypairs.*/launch) work=${0%/launch} ;;
    *)
        work=$(mktemp -d /var/tmp/primitive-memorypairs.XXXXXX) || exit 1
        if ! cp "$0" "$work/launch"; then
            rmdir "$work"
            exit 1
        fi
        cd / || exit 1
        exec /bin/sh "$work/launch"
        ;;
esac
trap 'rm -rf "$work"' EXIT
cp "$app" "$work/run" && chmod 755 "$work/run" || exit 1
"$work/run"
