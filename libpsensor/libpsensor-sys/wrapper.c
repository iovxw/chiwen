#include <psensor.h>

#include <amd.h>
#include <nvidia.h>
#include <lmsensor.h>
#include <hdd.h>
#include <pudisks2.h>

// #include <pgtop2.h>

bool psensor_amd_is_supported(void) { return amd_is_supported(); }
void psensor_amd_list_update(struct psensor **s) { amd_psensor_list_update(s); }
void psensor_amd_list_append(struct psensor ***s, int n) { amd_psensor_list_append(s, n); }
void psensor_amd_cleanup(void) { amd_cleanup(); }

bool psensor_nvidia_is_supported(void) { return nvidia_is_supported(); }
void psensor_nvidia_list_update(struct psensor **s) { nvidia_psensor_list_update(s); }
void psensor_nvidia_list_append(struct psensor ***s, int n) { nvidia_psensor_list_append(s, n); }
void psensor_nvidia_cleanup(void) { nvidia_cleanup(); }

bool psensor_lmsensor_is_supported(void) { return lmsensor_is_supported(); }
void psensor_lmsensor_list_update(struct psensor **s) { lmsensor_psensor_list_update(s); }
void psensor_lmsensor_list_append(struct psensor ***s, int n) { lmsensor_psensor_list_append(s, n); }
void psensor_lmsensor_cleanup(void) { lmsensor_cleanup(); }

bool psensor_atasmart_is_supported(void) { return atasmart_is_supported(); }
void psensor_atasmart_list_append(struct psensor ***s, int n) { atasmart_psensor_list_append(s, n); }
void psensor_atasmart_list_update(struct psensor **s) { atasmart_psensor_list_update(s); }

void psensor_hddtemp_list_append(struct psensor ***s, int n) { hddtemp_psensor_list_append(s, n); }
void psensor_hddtemp_list_update(struct psensor **s) { hddtemp_psensor_list_update(s); }

bool psensor_udisks2_is_supported(void) { return udisks2_is_supported(); }
void psensor_udisks2_list_append(struct psensor ***s, int n) { udisks2_psensor_list_append(s, n); }
void psensor_udisks2_list_update(struct psensor ** s) { udisks2_psensor_list_update(s); }
