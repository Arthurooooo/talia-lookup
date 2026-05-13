// Types miroir des structs Rust (src-tauri/src/bubble.rs).
// serde renvoie en camelCase via #[serde(rename_all = "camelCase")] côté Rust.

export interface BubbleUser {
  id: string;
  firstName?: string;
  lastName?: string;
  apiFullName?: string;
  email?: string;
  phoneNumber?: string;
  studentStatus?: string;
  leadStatus?: string;
  etatLead?: string;
  leadSource?: string;
  leadRating?: number;
  role?: string;
  mondayId?: string;
  birthDate?: string;
  becameStudentDate?: string;
  activatedDate?: string;
  lastLoginDate?: string;
  currentTrainingEndDate?: string;
  profilePicture?: string;
  gender?: string;
  nationality?: string;
  accountManagerId?: string;
  lastCallDate?: string;
  followUpCallDate?: string;
  authentication?: { email?: { email?: string } };
}

export interface BubbleContract {
  id: string;
  status?: string;
  startDate?: string;
  endDate?: string;
  createdDate?: string;
  oneMonthVerificationDate?: string;
  terminationDate?: string;
  terminationReason?: string;
  isTerminated?: boolean;
  isCancelled?: boolean;
  isTerminationOccurred?: boolean;
  isTerminationAccepted?: boolean;
  isTerminationSigned?: boolean;
  isTerminationSubmittedToOpco?: boolean;
  contractAmount?: number;
  trainingAmount?: number;
  numberOfHoursInElearning?: number;
  numberOfHoursInVirtualClassroom?: number;
  studentId?: string;
  classId?: string;
  mondayId?: string;
  trainingId?: string;
  companyId?: string;
  tutorId?: string;
}

export interface BubbleSession {
  id: string;
  startDate?: string;
  endDate?: string;
  studentId?: string;
  trainingId?: string;
  mainTeacherId?: string;
  pedagogicalReferentId?: string;
  schoolDay?: string;
  trimester?: string;
  year?: string;
  sessionType?: string;
  requiredHours?: string;
  regularTickets?: number;
  remainingRegularTickets?: number;
  examTickets?: number;
  remainingExamTickets?: number;
  absenceWarnings?: number;
  isInClass?: boolean;
  disabledDate?: string;
  examDate?: string;
  teacherAssignmentDate?: string;
  tutorCompanyId?: string;
}

export interface BubbleMeeting {
  id: string;
  startDate?: string;
  endDate?: string;
  meetingType?: string;
  meetingStatus?: string;
  attendeeIds?: string[];
  organizerId?: string;
  accountManagerId?: string;
  notes?: string;
}

export interface BubbleClass {
  id: string;
  name?: string;
  startDate?: string;
  endDate?: string;
  schoolDay?: string;
  teacherIds?: string[];
  classManagerId?: string;
  examDate?: string;
}

export interface StudentBundle {
  contracts: BubbleContract[];
  sessions: BubbleSession[];
  meetings: BubbleMeeting[];
  accountManager?: BubbleUser;
  activeTeacher?: BubbleUser;
  activeClass?: BubbleClass;
}

export interface RecentUserStub {
  id: string;
  fullName: string;
  subtitle?: string;
  avatarUrl?: string;
  visitedAt: string;
}

export function fullName(u: BubbleUser): string {
  const parts = [u.firstName, u.lastName].filter(s => s && s.trim().length);
  if (parts.length) return parts.join(' ');
  if (u.apiFullName) return u.apiFullName;
  return '(sans nom)';
}

export function bestEmail(u: BubbleUser): string | undefined {
  if (u.email) return u.email;
  return u.authentication?.email?.email ?? undefined;
}

export function initials(u: BubbleUser): string {
  const f = (u.firstName ?? '').trim().charAt(0);
  const l = (u.lastName ?? '').trim().charAt(0);
  const combo = (f + l).toUpperCase();
  return combo || '?';
}

export function initialsFromName(name: string): string {
  const parts = name.split(/\s+/).filter(Boolean);
  const f = parts[0]?.charAt(0) ?? '';
  const l = parts.slice(1).pop()?.charAt(0) ?? '';
  const combo = (f + l).toUpperCase();
  return combo || '?';
}
