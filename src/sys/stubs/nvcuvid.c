#include <nvcuvid.h>
CUresult CUDAAPI cuvidCreateVideoSource(CUvideosource *pObj, const char *pszFileName, CUVIDSOURCEPARAMS *pParams) { return 0; }
CUresult CUDAAPI cuvidCreateVideoSourceW(CUvideosource *pObj, const wchar_t *pwszFileName, CUVIDSOURCEPARAMS *pParams) { return 0; }
CUresult CUDAAPI cuvidDestroyVideoSource(CUvideosource obj) { return 0; }
CUresult CUDAAPI cuvidSetVideoSourceState(CUvideosource obj, cudaVideoState state) { return 0; }
CUresult CUDAAPI cuvidGetSourceVideoFormat(CUvideosource obj, CUVIDEOFORMAT *pvidfmt, unsigned int flags) { return 0; }
CUresult CUDAAPI cuvidGetSourceAudioFormat(CUvideosource obj, CUAUDIOFORMAT *paudfmt, unsigned int flags) { return 0; }
CUresult CUDAAPI cuvidCreateVideoParser(CUvideoparser *pObj, CUVIDPARSERPARAMS *pParams) { return 0; }
CUresult CUDAAPI cuvidParseVideoData(CUvideoparser obj, CUVIDSOURCEDATAPACKET *pPacket) { return 0; }
CUresult CUDAAPI cuvidDestroyVideoParser(CUvideoparser obj) { return 0; }